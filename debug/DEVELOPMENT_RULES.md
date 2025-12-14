# Development Rules

## Git - ОБОВ'ЯЗКОВО!

**Після кожної сесії роботи:**
1. `git add -A`
2. `git commit -m "опис змін"`
3. `git push`

**НЕ ЗАБУВАЙ КОМІТИТИ!** Результати роботи мають зберігатися в git!

## Rule 1: No Unauthorized Changes

**NEVER add or remove functionality unless the user explicitly asks for it.**

Examples of violations:
- Adding Q/E rotation controls when only asked to debug rotation
- Removing existing features while debugging
- Adding "improvements" that weren't requested

When debugging:
- Only add debug logging/println
- Don't change game logic
- Don't add new controls
- Don't remove existing code (only comment if needed for testing)

## Rule 2: Preserve Existing Behavior

When fixing bugs:
- Fix the specific bug only
- Don't refactor surrounding code
- Don't "improve" related systems
- Test that existing features still work

## Rule 3: Ask Before Major Changes

If a fix requires changing multiple systems:
- Explain what needs to change
- Wait for user approval
- Implement only what was approved

## Rule 4: Debug Logs Are Temporary

- Debug println/log statements should be clearly marked
- Remove them after debugging is complete
- Don't leave debug code in production
