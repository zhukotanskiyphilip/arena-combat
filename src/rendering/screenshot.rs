//! Screenshot capture module
//!
//! Captures the current frame and saves it as PNG image.
//! Used for visual debugging and AI analysis of ragdoll poses.

use wgpu;
use std::path::Path;

/// Captures a screenshot from GPU texture and saves to file
pub struct ScreenshotCapture {
    /// Buffer for reading pixels from GPU
    buffer: wgpu::Buffer,
    /// Width of capture
    width: u32,
    /// Height of capture
    height: u32,
    /// Bytes per row (aligned to 256)
    padded_bytes_per_row: u32,
    /// Unpadded bytes per row
    unpadded_bytes_per_row: u32,
}

impl ScreenshotCapture {
    /// Create new screenshot capture for given dimensions
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let bytes_per_pixel = 4u32; // RGBA
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        // wgpu requires rows aligned to 256 bytes
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padded_bytes_per_row = (unpadded_bytes_per_row + align - 1) / align * align;

        let buffer_size = (padded_bytes_per_row * height) as u64;

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Screenshot Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        Self {
            buffer,
            width,
            height,
            padded_bytes_per_row,
            unpadded_bytes_per_row,
        }
    }

    /// Copy texture to buffer (call after render, before present)
    pub fn copy_texture_to_buffer(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        texture: &wgpu::Texture,
    ) {
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &self.buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(self.padded_bytes_per_row),
                    rows_per_image: Some(self.height),
                },
            },
            wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
        );
    }

    /// Save buffer contents to PNG file (async - call after submit)
    pub fn save_to_file(&self, device: &wgpu::Device, path: &Path) -> Result<(), String> {
        let buffer_slice = self.buffer.slice(..);

        // Map buffer for reading
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });

        // Wait for GPU
        device.poll(wgpu::Maintain::Wait);

        // Check result
        rx.recv()
            .map_err(|e| format!("Failed to receive map result: {}", e))?
            .map_err(|e| format!("Failed to map buffer: {:?}", e))?;

        // Read data
        let data = buffer_slice.get_mapped_range();

        // Remove padding from rows
        let mut pixels = Vec::with_capacity((self.width * self.height * 4) as usize);
        for row in 0..self.height {
            let start = (row * self.padded_bytes_per_row) as usize;
            let end = start + self.unpadded_bytes_per_row as usize;
            pixels.extend_from_slice(&data[start..end]);
        }

        drop(data);
        self.buffer.unmap();

        // Convert BGRA to RGBA (wgpu uses BGRA on some platforms)
        for chunk in pixels.chunks_exact_mut(4) {
            chunk.swap(0, 2); // Swap B and R
        }

        // Save as PNG
        image::save_buffer(
            path,
            &pixels,
            self.width,
            self.height,
            image::ColorType::Rgba8,
        ).map_err(|e| format!("Failed to save image: {}", e))?;

        Ok(())
    }
}

/// Helper to capture first frame
pub struct FirstFrameCapture {
    capture: Option<ScreenshotCapture>,
    captured: bool,
    frame_count: u32,
    target_frame: u32,
}

impl FirstFrameCapture {
    pub fn new() -> Self {
        Self {
            capture: None,
            captured: false,
            frame_count: 0,
            target_frame: 5, // Wait a few frames for stable pose
        }
    }

    /// Initialize capture (call once when device/size known)
    pub fn init(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        if self.capture.is_none() {
            self.capture = Some(ScreenshotCapture::new(device, width, height));
        }
    }

    /// Check if should capture this frame
    pub fn should_capture(&mut self) -> bool {
        self.frame_count += 1;
        !self.captured && self.frame_count == self.target_frame
    }

    /// Copy texture (call in render, before submit)
    pub fn copy_if_needed(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        texture: &wgpu::Texture,
    ) {
        if self.frame_count == self.target_frame && !self.captured {
            if let Some(ref capture) = self.capture {
                capture.copy_texture_to_buffer(encoder, texture);
            }
        }
    }

    /// Save screenshot (call after submit)
    pub fn save_if_needed(&mut self, device: &wgpu::Device) {
        if self.frame_count == self.target_frame && !self.captured {
            if let Some(ref capture) = self.capture {
                let path = std::path::Path::new("debug/ragdoll_frame1.png");
                match capture.save_to_file(device, path) {
                    Ok(()) => {
                        log::info!("Screenshot saved to {:?}", path);
                        self.captured = true;
                    }
                    Err(e) => {
                        log::error!("Failed to save screenshot: {}", e);
                    }
                }
            }
        }
    }

    pub fn is_captured(&self) -> bool {
        self.captured
    }
}
