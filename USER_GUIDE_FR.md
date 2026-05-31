# Guide Utilisateur

NewGPA est une interface moderne pour gérer des clés OpenPGP, certificats S/MIME, fichiers chiffrés, signatures et futurs coffres locaux sécurisés.

## Principes

- Les mots de passe et passphrases sont demandés par `pinentry`.
- Le réseau est désactivé par défaut.
- Le module Post-Quantum Lab est expérimental et désactivé par défaut.

## Démarrage

Lancer l'application:

```bash
newgpa
```

Diagnostic:

```bash
newgpa doctor
```

## Sections

- Mes clés: clés OpenPGP publiques et privées.
- Certificats: certificats X.509/S/MIME.
- Chiffrer: sélection de fichiers et destinataires.
- Déchiffrer: déchiffrement via GnuPG.
- Signer: signatures attachées, détachées ou clearsign.
- Vérifier: rapports de signature.
- Coffre sécurisé: espace local chiffré prévu.
- Post-Quantum Lab: expérimental, non compatible OpenPGP standard.
- Paramètres: sécurité, réseau, proxy, apparence.

