# GPA vs Kleopatra

## GPA

Strengths:

- Lightweight GnuPG assistant.
- Direct OpenPGP key and file actions.
- Simple mental model for individual users.

Limits:

- Older GTK interface.
- Limited S/MIME and enterprise certificate workflows.
- Less guidance for signature trust decisions.
- Limited modern packaging story.

## Kleopatra

Strengths:

- Strong OpenPGP and S/MIME coverage.
- Certificate chain and enterprise workflows.
- Clearer verification and recipient selection.
- Smartcard and certificate management integration.

Limits:

- KDE/PIM ecosystem dependency footprint.
- Heavier for users who want a portable standalone tool.

## NewGPA Direction

NewGPA should keep GPA's directness, add Kleopatra-style certificate workflows, and remain portable through AppImage. It must use standard GnuPG infrastructure for compatible crypto and isolate experimental post-quantum work.

