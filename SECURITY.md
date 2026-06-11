# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.3.x | Yes |
| 0.2.x | No |
| < 0.2 | No |

## Reporting a Vulnerability

If you discover a security vulnerability in SourisDW, please report it responsibly.

**Do NOT open a public GitHub issue for security vulnerabilities.**

Instead, please send an email to the project maintainer with:

1. Description of the vulnerability
2. Steps to reproduce
3. Potential impact
4. Suggested fix (if any)

You should receive a response within 48 hours acknowledging receipt.

## What to Expect

- Acknowledgment within 48 hours
- Assessment of the vulnerability within 7 days
- Fix or mitigation plan shared with reporter before public disclosure
- Public disclosure after fix is released

## Scope

Security issues that are in scope:

- Command injection through URL inputs
- Path traversal in output directory
- Unsafe file permissions on downloaded files
- Dependency vulnerabilities (yt-dlp, ffmpeg)
- Credential exposure in logs or config files

Out of scope:

- Issues in yt-dlp or ffmpeg themselves (report to their maintainers)
- Issues requiring physical access to the user's machine
- Social engineering attacks

## Best Practices for Users

- Keep SourisDW updated to the latest version
- Review config files for sensitive information before sharing
- Use the `--no-auto-update` flag if you need reproducible behavior
- Run with minimal privileges when possible
