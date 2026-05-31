use adw::prelude::*;
use anyhow::Result;
use std::{
    cell::RefCell,
    env, fs,
    io::Write,
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{Command, Stdio},
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};

static ENGLISH_UI: AtomicBool = AtomicBool::new(true);

pub fn run() -> Result<()> {
    adw::init()?;
    ENGLISH_UI.store(default_english_ui(), Ordering::Relaxed);
    let app = adw::Application::builder()
        .application_id("org.newgpa.NewGPA")
        .build();
    app.connect_activate(build_ui);
    app.run();
    Ok(())
}

fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("NewGPA")
        .default_width(1260)
        .default_height(820)
        .build();
    set_window_content(&window);
    window.present();
}

fn set_window_content(window: &adw::ApplicationWindow) {
    let header = adw::HeaderBar::new();
    let language = button(language_button_label());
    language.set_tooltip_text(Some(tr("Changer de langue", "Change Language")));
    header.pack_start(&language);

    let title = gtk4::Label::new(Some("NewGPA"));
    title.add_css_class("title-1");
    header.set_title_widget(Some(&title));

    let tabs = gtk4::Stack::new();
    tabs.set_hexpand(true);
    tabs.set_vexpand(true);
    tabs.set_transition_type(gtk4::StackTransitionType::SlideLeftRight);
    tabs.add_titled(
        &keys_page(),
        Some("keys"),
        tr("Clés OpenPGP", "OpenPGP Keys"),
    );
    tabs.add_titled(
        &certificates_page(),
        Some("certificates"),
        tr("Certificats X.509", "X.509 Certificates"),
    );
    tabs.add_titled(&files_page(), Some("files"), tr("Fichiers", "Files"));
    tabs.add_titled(
        &clipboard_page(),
        Some("clipboard"),
        tr("Presse-papiers", "Clipboard"),
    );
    tabs.add_titled(
        &card_page(),
        Some("card"),
        tr("Carte / token", "Card / Token"),
    );
    tabs.add_titled(
        &servers_page(),
        Some("servers"),
        tr("Serveurs de clés", "Key Servers"),
    );
    tabs.add_titled(
        &vault_page(),
        Some("vault"),
        tr("Coffre sécurisé", "Secure Vault"),
    );
    tabs.add_titled(&post_quantum_page(), Some("pq"), "Post-Quantum Lab");
    tabs.add_titled(
        &settings_page(),
        Some("settings"),
        tr("Paramètres", "Settings"),
    );

    let switcher = gtk4::StackSidebar::new();
    switcher.set_stack(&tabs);
    switcher.set_width_request(230);

    let content = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    content.set_hexpand(true);
    content.set_vexpand(true);
    content.append(&switcher);
    content.append(&tabs);

    let root = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    root.set_hexpand(true);
    root.set_vexpand(true);
    root.append(&header);
    root.append(&content);

    {
        let window = window.clone();
        language.connect_clicked(move |_| {
            ENGLISH_UI.fetch_xor(true, Ordering::Relaxed);
            set_window_content(&window);
        });
    }

    window.set_content(Some(&root));
}

fn keys_page() -> gtk4::ScrolledWindow {
    let page = page(tr("Gestion des clés OpenPGP", "OpenPGP Key Management"));
    let output = text_output(false);
    let status = status_label();

    let (import_picker, import_path) = path_picker(
        tr(
            "Clé ou ownertrust à importer",
            "Key or ownertrust file to import",
        ),
        gtk4::FileChooserAction::Open,
    );
    let (export_picker, export_path) = path_picker(
        tr("Export public .asc", "Public export .asc"),
        gtk4::FileChooserAction::Save,
    );
    let (secret_picker, secret_path) = path_picker(
        tr("Export secret .asc", "Secret export .asc"),
        gtk4::FileChooserAction::Save,
    );
    let uid = entry(tr(
        "Nouvelle clé: Nom <email@example.org>",
        "New key: Name <email@example.org>",
    ));
    let expiry = entry(tr(
        "Expiration: 2y, 1y, 6m, never",
        "Expiration: 2y, 1y, 6m, never",
    ));
    expiry.set_text("2y");
    let (revoke_picker, revoke_path) = path_picker(
        tr(
            "Certificat de révocation .rev",
            "Revocation certificate .rev",
        ),
        gtk4::FileChooserAction::Save,
    );
    let key_list = gtk4::ListBox::new();
    key_list.set_selection_mode(gtk4::SelectionMode::Single);
    key_list.add_css_class("boxed-list");
    let selected_key = Rc::new(RefCell::new(String::new()));
    let selected_label = gtk4::Label::new(Some(tr("Aucune clé sélectionnée", "No key selected")));
    selected_label.set_xalign(0.0);
    selected_label.add_css_class("dim-label");

    let refresh = button(tr("Actualiser", "Refresh"));
    let private_keys = button(tr("Clés privées", "Private Keys"));
    let fingerprints = button(tr("Empreintes", "Fingerprints"));
    let import = button(tr("Importer", "Import"));
    let export_public = button(tr("Exporter public", "Export Public"));
    let export_secret = button(tr("Exporter secret", "Export Secret"));
    let import_ownertrust = button(tr("Importer ownertrust", "Import Ownertrust"));
    let export_ownertrust = button(tr("Exporter ownertrust", "Export Ownertrust"));
    let create = button(tr("Créer clé", "Create Key"));
    let revoke = button(tr("Générer révocation", "Generate Revocation"));
    let trust = button(tr("Appliquer confiance", "Apply Trust"));
    let trust_level = trust_combo();
    let disable = button(tr("Désactiver clé", "Disable Key"));
    let delete_public = button(tr("Supprimer public", "Delete Public"));
    let delete_secret = button(tr("Supprimer secret", "Delete Secret"));

    refresh_keys(&output, &status);
    populate_key_list(&key_list, &selected_key, &selected_label, false);
    {
        let selected_key = selected_key.clone();
        let selected_label = selected_label.clone();
        key_list.connect_row_selected(move |_, row| {
            if let Some(row) = row {
                let key = row.widget_name().to_string();
                *selected_key.borrow_mut() = key.clone();
                selected_label.set_text(&format!(
                    "{}: {key}",
                    tr("Clé sélectionnée", "Selected key")
                ));
            }
        });
    }

    connect_output(&refresh, &output, &status, || {
        run_gpg(&["--list-keys", "--keyid-format", "long"])
    });
    {
        let key_list = key_list.clone();
        let selected_key = selected_key.clone();
        let selected_label = selected_label.clone();
        refresh.connect_clicked(move |_| {
            populate_key_list(&key_list, &selected_key, &selected_label, false)
        });
    }
    connect_output(&private_keys, &output, &status, || {
        run_gpg(&["--list-secret-keys", "--keyid-format", "long"])
    });
    connect_output(&fingerprints, &output, &status, || {
        run_gpg(&["--fingerprint", "--fingerprint", "--keyid-format", "long"])
    });
    connect_status(&import, &status, {
        let import_path = import_path.clone();
        move || run_gpg(&["--import", &selected_path(&import_path)?])
    });
    {
        let key_list = key_list.clone();
        let selected_key = selected_key.clone();
        let selected_label = selected_label.clone();
        import.connect_clicked(move |_| {
            populate_key_list(&key_list, &selected_key, &selected_label, false)
        });
    }
    connect_status(&export_public, &status, {
        let export_path = export_path.clone();
        let selected_key = selected_key.clone();
        move || {
            let key = selected_key_id(&selected_key)?;
            run_gpg(&[
                "--armor",
                "--output",
                &selected_path(&export_path)?,
                "--export",
                &key,
            ])
        }
    });
    connect_status(&export_secret, &status, {
        let secret_path = secret_path.clone();
        let selected_key = selected_key.clone();
        move || {
            let key = selected_key_id(&selected_key)?;
            run_gpg(&[
                "--armor",
                "--output",
                &selected_path(&secret_path)?,
                "--export-secret-keys",
                &key,
            ])
        }
    });
    connect_status(&import_ownertrust, &status, {
        let import_path = import_path.clone();
        move || run_gpg(&["--import-ownertrust", &selected_path(&import_path)?])
    });
    connect_status(&export_ownertrust, &status, {
        let export_path = export_path.clone();
        move || {
            run_program_to_file(
                "gpg",
                &["--export-ownertrust".to_string()],
                &selected_path(&export_path)?,
                0o600,
            )
        }
    });
    connect_status(&create, &status, {
        let uid = uid.clone();
        let expiry = expiry.clone();
        move || {
            run_gpg(&[
                "--quick-generate-key",
                &uid.text(),
                "future-default",
                "default",
                &expiry.text(),
            ])
        }
    });
    {
        let key_list = key_list.clone();
        let selected_key = selected_key.clone();
        let selected_label = selected_label.clone();
        create.connect_clicked(move |_| {
            populate_key_list(&key_list, &selected_key, &selected_label, false)
        });
    }
    connect_status(&revoke, &status, {
        let selected_key = selected_key.clone();
        let revoke_path = revoke_path.clone();
        move || {
            let key = selected_key_id(&selected_key)?;
            run_gpg(&[
                "--output",
                &selected_path(&revoke_path)?,
                "--gen-revoke",
                &key,
            ])
        }
    });
    connect_status(&trust, &status, {
        let selected_key = selected_key.clone();
        let trust_level = trust_level.clone();
        move || {
            run_gpg(&[
                "--quick-set-ownertrust",
                &selected_key_id(&selected_key)?,
                &selected_id(&trust_level)?,
            ])
        }
    });
    connect_status(&disable, &status, {
        let selected_key = selected_key.clone();
        move || {
            run_gpg(&[
                "--edit-key",
                &selected_key_id(&selected_key)?,
                "disable",
                "quit",
            ])
        }
    });
    connect_status(&delete_public, &status, {
        let selected_key = selected_key.clone();
        move || {
            run_gpg(&[
                "--batch",
                "--yes",
                "--delete-keys",
                &selected_key_id(&selected_key)?,
            ])
        }
    });
    {
        let key_list = key_list.clone();
        let selected_key = selected_key.clone();
        let selected_label = selected_label.clone();
        delete_public.connect_clicked(move |_| {
            populate_key_list(&key_list, &selected_key, &selected_label, false)
        });
    }
    connect_status(&delete_secret, &status, {
        let selected_key = selected_key.clone();
        move || {
            run_gpg(&[
                "--batch",
                "--yes",
                "--delete-secret-keys",
                &selected_key_id(&selected_key)?,
            ])
        }
    });
    {
        let key_list = key_list.clone();
        let selected_key = selected_key.clone();
        let selected_label = selected_label.clone();
        delete_secret.connect_clicked(move |_| {
            populate_key_list(&key_list, &selected_key, &selected_label, false)
        });
    }

    page.append(&group(
        tr("Inventaire", "Inventory"),
        vec![
            widget(&button_row(&[&refresh, &private_keys, &fingerprints])),
            widget(&scroll(&key_list, 280)),
            widget(&selected_label),
        ],
    ));
    page.append(&group(
        tr("Création et révocation", "Creation and Revocation"),
        vec![
            widget(&uid),
            widget(&expiry),
            widget(&create),
            widget(&revoke_picker),
            widget(&revoke),
        ],
    ));
    page.append(&group(
        tr("Import / export", "Import / Export"),
        vec![
            widget(&import_picker),
            widget(&button_row(&[&import, &import_ownertrust])),
            widget(&export_picker),
            widget(&secret_picker),
            widget(&button_row(&[
                &export_public,
                &export_secret,
                &export_ownertrust,
            ])),
        ],
    ));
    page.append(&group(
        tr("Maintenance", "Maintenance"),
        vec![
            widget(&trust_level),
            widget(&button_row(&[
                &trust,
                &disable,
                &delete_public,
                &delete_secret,
            ])),
        ],
    ));
    page.append(&status);
    page.append(&scroll(&output, 260));
    page_scroller(&page)
}

fn certificates_page() -> gtk4::ScrolledWindow {
    let page = page(tr(
        "Certificats S/MIME / X.509",
        "S/MIME / X.509 Certificates",
    ));
    let output = text_output(false);
    let status = status_label();
    let (cert_picker, cert_path) = path_picker(
        tr(
            "Certificat à importer/exporter",
            "Certificate to import/export",
        ),
        gtk4::FileChooserAction::Open,
    );
    let (cert_save_picker, cert_save_path) = path_picker(
        tr(
            "Export certificat .pem/.asc",
            "Certificate export .pem/.asc",
        ),
        gtk4::FileChooserAction::Save,
    );
    let cert_select = certificate_combo();

    let list = button(tr("Lister certificats", "List Certificates"));
    let private = button(tr("Certificats privés", "Private Certificates"));
    let import = button(tr("Importer", "Import"));
    let export = button(tr("Exporter", "Export"));
    let delete = button(tr("Supprimer", "Delete"));
    let verify_chain = button(tr("Vérifier chaîne", "Verify Chain"));

    connect_output(&list, &output, &status, || {
        run_gpgsm(&["--list-keys", "--with-validation"])
    });
    {
        let cert_select = cert_select.clone();
        list.connect_clicked(move |_| populate_certificate_combo(&cert_select));
    }
    connect_output(&private, &output, &status, || {
        run_gpgsm(&["--list-secret-keys"])
    });
    connect_status(&import, &status, {
        let cert_path = cert_path.clone();
        move || run_gpgsm(&["--import", &selected_path(&cert_path)?])
    });
    connect_status(&export, &status, {
        let cert_save_path = cert_save_path.clone();
        let cert_select = cert_select.clone();
        move || {
            let cert = selected_id(&cert_select)?;
            run_gpgsm(&[
                "--armor",
                "--output",
                &selected_path(&cert_save_path)?,
                "--export",
                &cert,
            ])
        }
    });
    connect_status(&delete, &status, {
        let cert_select = cert_select.clone();
        move || run_gpgsm(&["--batch", "--delete-keys", &selected_id(&cert_select)?])
    });
    connect_output(&verify_chain, &output, &status, {
        let cert_select = cert_select.clone();
        move || {
            run_gpgsm(&[
                "--with-validation",
                "--list-keys",
                &selected_id(&cert_select)?,
            ])
        }
    });

    page.append(&group(
        tr("Inventaire X.509", "X.509 Inventory"),
        vec![widget(&button_row(&[&list, &private, &verify_chain]))],
    ));
    page.append(&group(
        tr("Import / export / suppression", "Import / Export / Delete"),
        vec![
            widget(&cert_select),
            widget(&cert_picker),
            widget(&cert_save_picker),
            widget(&button_row(&[&import, &export, &delete])),
        ],
    ));
    page.append(&status);
    page.append(&scroll(&output, 320));
    page_scroller(&page)
}

fn files_page() -> gtk4::ScrolledWindow {
    let page = page(tr("Opérations sur fichiers", "File Operations"));
    let status = status_label();
    let report = text_output(false);

    let (input_picker, input) = path_picker(
        tr("Fichier source", "Source File"),
        gtk4::FileChooserAction::Open,
    );
    let (output_picker, output) = path_picker(
        tr("Fichier de sortie", "Output File"),
        gtk4::FileChooserAction::Save,
    );
    let recipient_select = key_combo(false);
    let (signature_picker, signature) = path_picker(
        tr("Signature ou fichier signé", "Signature or Signed File"),
        gtk4::FileChooserAction::Open,
    );
    let (signed_original_picker, signed_original) = path_picker(
        tr("Original signé optionnel", "Optional Signed Original"),
        gtk4::FileChooserAction::Open,
    );
    let armor = gtk4::CheckButton::with_label("ASCII armor");
    armor.set_active(true);
    let sign =
        gtk4::CheckButton::with_label(tr("Signer pendant le chiffrement", "Sign while encrypting"));
    let symmetric =
        gtk4::CheckButton::with_label(tr("Chiffrement symétrique", "Symmetric Encryption"));
    let detached = gtk4::CheckButton::with_label(tr("Signature détachée", "Detached Signature"));
    detached.set_active(true);
    let clearsign = gtk4::CheckButton::with_label("Clearsign texte");

    let encrypt = button(tr("Chiffrer", "Encrypt"));
    let decrypt = button(tr("Déchiffrer", "Decrypt"));
    let sign_file = button(tr("Signer", "Sign"));
    let verify = button(tr("Vérifier", "Verify"));

    connect_status(&encrypt, &status, {
        let input = input.clone();
        let output = output.clone();
        let recipient_select = recipient_select.clone();
        let armor = armor.clone();
        let sign = sign.clone();
        let symmetric = symmetric.clone();
        move || {
            let mut args = vec![
                "--yes".to_string(),
                "--output".to_string(),
                selected_path(&output)?,
            ];
            if armor.is_active() {
                args.push("--armor".into());
            }
            if symmetric.is_active() {
                args.push("--symmetric".into());
            } else {
                args.push("--encrypt".into());
                args.push("--recipient".into());
                args.push(selected_id(&recipient_select)?);
            }
            if sign.is_active() {
                args.push("--sign".into());
            }
            args.push(selected_path(&input)?);
            run_program("gpg", &args)
        }
    });
    connect_status(&decrypt, &status, {
        let input = input.clone();
        let output = output.clone();
        move || {
            run_gpg(&[
                "--yes",
                "--decrypt",
                "--output",
                &selected_path(&output)?,
                &selected_path(&input)?,
            ])
        }
    });
    connect_status(&sign_file, &status, {
        let input = input.clone();
        let output = output.clone();
        let detached = detached.clone();
        let clearsign = clearsign.clone();
        move || {
            let mode = if clearsign.is_active() {
                "--clearsign"
            } else if detached.is_active() {
                "--detach-sign"
            } else {
                "--sign"
            };
            run_gpg(&[
                "--yes",
                mode,
                "--output",
                &selected_path(&output)?,
                &selected_path(&input)?,
            ])
        }
    });
    connect_output(&verify, &report, &status, {
        let signature = signature.clone();
        let signed_original = signed_original.clone();
        move || {
            let mut args = vec!["--verify".to_string(), selected_path(&signature)?];
            if !signed_original.borrow().is_empty() {
                args.push(selected_path(&signed_original)?);
            }
            run_program("gpg", &args)
        }
    });

    page.append(&group(
        tr("Entrée / sortie", "Input / Output"),
        vec![
            widget(&input_picker),
            widget(&output_picker),
            widget(&recipient_select),
            widget(&button_row(&[&armor, &sign, &symmetric])),
        ],
    ));
    page.append(&group(
        tr("Chiffrer / déchiffrer / signer", "Encrypt / Decrypt / Sign"),
        vec![
            widget(&button_row(&[&encrypt, &decrypt, &sign_file])),
            widget(&button_row(&[&detached, &clearsign])),
        ],
    ));
    page.append(&group(
        tr("Vérification", "Verification"),
        vec![
            widget(&signature_picker),
            widget(&signed_original_picker),
            widget(&verify),
        ],
    ));
    page.append(&status);
    page.append(&scroll(&report, 260));
    page_scroller(&page)
}

fn clipboard_page() -> gtk4::ScrolledWindow {
    let page = page(tr(
        "Presse-papiers compatible GPA",
        "GPA-Compatible Clipboard",
    ));
    let input = text_output(true);
    let output = text_output(true);
    let status = status_label();
    let recipients = entry(tr(
        "Destinataires pour chiffrement texte",
        "Recipients for text encryption",
    ));

    let encrypt = button(tr("Chiffrer texte", "Encrypt Text"));
    let decrypt = button(tr("Déchiffrer texte", "Decrypt Text"));
    let sign = button(tr("Signer texte", "Sign Text"));
    let verify = button(tr("Vérifier texte", "Verify Text"));
    let clear = button(tr("Effacer", "Clear"));

    connect_text_transform(&encrypt, &input, &output, &status, {
        let recipients = recipients.clone();
        move |text| {
            let mut args = vec!["--armor".to_string(), "--encrypt".to_string()];
            for recipient in split_list(&recipients.text()) {
                args.push("--recipient".into());
                args.push(recipient);
            }
            run_program_with_input("gpg", &args, text)
        }
    });
    connect_text_transform(&decrypt, &input, &output, &status, |text| {
        run_program_with_input("gpg", &["--decrypt".to_string()], text)
    });
    connect_text_transform(&sign, &input, &output, &status, |text| {
        run_program_with_input(
            "gpg",
            &["--armor".to_string(), "--clearsign".to_string()],
            text,
        )
    });
    connect_text_transform(&verify, &input, &output, &status, |text| {
        run_program_with_input("gpg", &["--verify".to_string()], text)
    });
    {
        let input = input.clone();
        let output = output.clone();
        let status = status.clone();
        clear.connect_clicked(move |_| {
            input.buffer().set_text("");
            output.buffer().set_text("");
            status.set_text(tr("Zones texte effacées.", "Text areas cleared."));
        });
    }

    page.append(&group(
        tr("Destinataires", "Recipients"),
        vec![widget(&recipients)],
    ));
    page.append(&group(
        tr("Actions", "Actions"),
        vec![widget(&button_row(&[
            &encrypt, &decrypt, &sign, &verify, &clear,
        ]))],
    ));
    page.append(&status);
    page.append(&split_text_views(
        tr("Texte source", "Source Text"),
        &input,
        tr("Résultat", "Result"),
        &output,
    ));
    page_scroller(&page)
}

fn card_page() -> gtk4::ScrolledWindow {
    let page = page(tr(
        "Carte à puce / token OpenPGP",
        "OpenPGP Smartcard / Token",
    ));
    let output = text_output(false);
    let status = status_label();

    let status_button = button(tr("État carte", "Card Status"));
    let learn = button(tr("Apprendre carte", "Learn Card"));
    let edit = button(tr("Menu carte", "Card Menu"));
    let agent_reload = button(tr("Recharger agent", "Reload Agent"));

    connect_output(&status_button, &output, &status, || {
        run_gpg(&["--card-status"])
    });
    connect_status(&learn, &status, || {
        run_gpg(&["--card-edit", "fetch", "quit"])
    });
    connect_output(&edit, &output, &status, || run_gpg(&["--card-edit"]));
    connect_status(&agent_reload, &status, || {
        run_program("gpg-connect-agent", &["reloadagent".into(), "/bye".into()])
    });

    page.append(&group(
        tr("Carte / token", "Card / Token"),
        vec![widget(&button_row(&[
            &status_button,
            &learn,
            &edit,
            &agent_reload,
        ]))],
    ));
    page.append(&status);
    page.append(&scroll(&output, 360));
    page_scroller(&page)
}

fn servers_page() -> gtk4::ScrolledWindow {
    let page = page(tr(
        "Serveurs de clés et recherche",
        "Key Servers and Search",
    ));
    let output = text_output(false);
    let status = status_label();
    let query = entry(tr(
        "Email, domaine, empreinte ou ID de clé",
        "Email, domain, fingerprint, or key ID",
    ));
    let keyserver = entry(tr(
        "Serveur optionnel, ex: hkps://keys.openpgp.org",
        "Optional server, e.g. hkps://keys.openpgp.org",
    ));

    let locate = button(tr("Recherche WKD", "WKD Lookup"));
    let search = button(tr("Chercher serveur", "Search Server"));
    let receive = button(tr("Recevoir clé", "Receive Key"));
    let send = button(tr("Envoyer clé", "Send Key"));
    let refresh = button(tr("Rafraîchir", "Refresh"));

    connect_output(&locate, &output, &status, {
        let query = query.clone();
        move || run_gpg(&["--locate-keys", &query.text()])
    });
    connect_output(&search, &output, &status, {
        let query = query.clone();
        let keyserver = keyserver.clone();
        move || run_gpg_with_keyserver(&keyserver.text(), &["--search-keys", &query.text()])
    });
    connect_status(&receive, &status, {
        let query = query.clone();
        let keyserver = keyserver.clone();
        move || run_gpg_with_keyserver(&keyserver.text(), &["--recv-keys", &query.text()])
    });
    connect_status(&send, &status, {
        let query = query.clone();
        let keyserver = keyserver.clone();
        move || run_gpg_with_keyserver(&keyserver.text(), &["--send-keys", &query.text()])
    });
    connect_status(&refresh, &status, {
        let query = query.clone();
        let keyserver = keyserver.clone();
        move || {
            if query.text().is_empty() {
                run_gpg_with_keyserver(&keyserver.text(), &["--refresh-keys"])
            } else {
                run_gpg_with_keyserver(&keyserver.text(), &["--refresh-keys", &query.text()])
            }
        }
    });

    page.append(&group(
        tr("Recherche", "Search"),
        vec![widget(&query), widget(&keyserver)],
    ));
    page.append(&group(
        tr("Actions réseau", "Network Actions"),
        vec![widget(&button_row(&[
            &locate, &search, &receive, &send, &refresh,
        ]))],
    ));
    page.append(&status);
    page.append(&scroll(&output, 360));
    page_scroller(&page)
}

fn vault_page() -> gtk4::ScrolledWindow {
    let page = page(tr("Coffre sécurisé local", "Local Secure Vault"));
    let (vault_picker, path) = path_picker(
        tr("Dossier coffre", "Vault Folder"),
        gtk4::FileChooserAction::SelectFolder,
    );
    let (archive_picker, archive) = path_picker(
        tr("Archive chiffrée .tar.gpg", "Encrypted Archive .tar.gpg"),
        gtk4::FileChooserAction::Save,
    );
    let status = status_label();
    let create = button(tr("Créer dossier 0700", "Create 0700 Folder"));
    let lock = button(tr("Archiver + chiffrer", "Archive + Encrypt"));

    connect_status(&create, &status, {
        let path = path.clone();
        move || create_vault_dir(&selected_path(&path)?)
    });
    connect_status(&lock, &status, {
        let path = path.clone();
        let archive = archive.clone();
        move || {
            let path = selected_path(&path)?;
            let archive = selected_path(&archive)?;
            reject_dangerous_path(Path::new(&path))?;
            reject_dangerous_path(Path::new(&archive))?;
            run_program(
                "sh",
                &[
                    "-c".into(),
                    "tar -C \"$1\" -cf - . | gpg --symmetric --cipher-algo AES256 --output \"$2\""
                        .into(),
                    "newgpa-vault".into(),
                    path,
                    archive,
                ],
            )
        }
    });

    page.append(&group(
        tr("Dossier", "Folder"),
        vec![
            widget(&vault_picker),
            widget(&archive_picker),
            widget(&button_row(&[&create, &lock])),
        ],
    ));
    page.append(&notice(
        tr(
            "Le coffre utilise les permissions Unix 0700 et un chiffrement symétrique GnuPG pour l’archive. La phrase secrète reste gérée par pinentry.",
            "The vault uses Unix 0700 permissions and symmetric GnuPG encryption for the archive. The passphrase remains handled by pinentry.",
        ),
    ));
    page.append(&status);
    page_scroller(&page)
}

fn post_quantum_page() -> gtk4::ScrolledWindow {
    let page = page("Post-Quantum Lab");
    let output = text_output(false);
    let status = status_label();
    let inspect = button(tr("État des outils PQ", "PQ Tool Status"));

    connect_output(&inspect, &output, &status, || {
        let mut text = String::new();
        for tool in ["oqsprovider", "openssl", "gpg"] {
            let found = command_exists(tool);
            text.push_str(&format!(
                "{tool}: {}\n",
                if found { "trouvé" } else { "absent" }
            ));
        }
        text.push_str(tr(
            "\nOpenPGP stable ne standardise pas encore ML-KEM/ML-DSA pour les clés GnuPG classiques. NewGPA expose donc ce mode comme laboratoire, désactivé par défaut.\n",
            "\nStable OpenPGP does not yet standardize ML-KEM/ML-DSA for classic GnuPG keys. NewGPA exposes this mode as a lab, disabled by default.\n",
        ));
        Ok(text)
    });

    page.append(&notice(
        tr(
            "Laboratoire expérimental. Les modes post-quantiques ne remplacent pas OpenPGP/GnuPG tant que l’interopérabilité n’est pas standardisée.",
            "Experimental lab. Post-quantum modes do not replace OpenPGP/GnuPG until interoperability is standardized.",
        ),
    ));
    page.append(&inspect);
    page.append(&status);
    page.append(&scroll(&output, 260));
    page_scroller(&page)
}

fn settings_page() -> gtk4::ScrolledWindow {
    let page = page(tr(
        "Paramètres et diagnostic GnuPG",
        "GnuPG Settings and Diagnostics",
    ));
    let output = text_output(false);
    let status = status_label();
    let doctor = button(tr("Diagnostic", "Diagnostics"));
    let components = button(tr("Composants", "Components"));
    let dirs = button(tr("Dossiers GnuPG", "GnuPG Directories"));
    let reload = button(tr("Recharger agent", "Reload Agent"));
    let kill_agent = button(tr("Redémarrer agent", "Restart Agent"));

    connect_output(&doctor, &output, &status, || {
        let mut text = String::from(
            tr(
                "NewGPA doctor\nHigh Security: activé\nRéseau automatique: désactivé\nLangue: français détecté ou choisi dans l'interface\n\n",
                "NewGPA doctor\nHigh Security: enabled\nAutomatic network: disabled\nLanguage: English detected or selected in the interface\n\n",
            ),
        );
        for tool in [
            "gpg",
            "gpgsm",
            "gpg-agent",
            "pinentry",
            "gpgconf",
            "gpg-connect-agent",
        ] {
            text.push_str(&format!(
                "{tool}: {}\n",
                if command_exists(tool) {
                    tr("trouvé", "found")
                } else {
                    tr("manquant", "missing")
                }
            ));
        }
        Ok(text)
    });
    connect_output(&components, &output, &status, || {
        run_program("gpgconf", &["--list-components".into()])
    });
    connect_output(&dirs, &output, &status, || {
        run_program("gpgconf", &["--list-dirs".into()])
    });
    connect_status(&reload, &status, || {
        run_program("gpg-connect-agent", &["reloadagent".into(), "/bye".into()])
    });
    connect_status(&kill_agent, &status, || {
        run_program("gpgconf", &["--kill".into(), "gpg-agent".into()])
    });

    page.append(&group(
        "GnuPG",
        vec![widget(&button_row(&[
            &doctor,
            &components,
            &dirs,
            &reload,
            &kill_agent,
        ]))],
    ));
    page.append(&status);
    page.append(&scroll(&output, 420));
    page_scroller(&page)
}

fn page(title: &str) -> gtk4::Box {
    let page = gtk4::Box::new(gtk4::Orientation::Vertical, 14);
    page.set_hexpand(true);
    page.set_vexpand(true);
    page.set_margin_top(18);
    page.set_margin_bottom(18);
    page.set_margin_start(22);
    page.set_margin_end(22);

    let label = gtk4::Label::new(Some(title));
    label.set_halign(gtk4::Align::Start);
    label.add_css_class("title-2");
    page.append(&label);
    page
}

fn page_scroller(page: &gtk4::Box) -> gtk4::ScrolledWindow {
    gtk4::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(page)
        .build()
}

fn group(title: &str, children: Vec<gtk4::Widget>) -> gtk4::Frame {
    let frame = gtk4::Frame::new(Some(title));
    frame.set_hexpand(true);
    let box_ = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    box_.set_margin_top(10);
    box_.set_margin_bottom(10);
    box_.set_margin_start(10);
    box_.set_margin_end(10);
    for child in children {
        box_.append(&child);
    }
    frame.set_child(Some(&box_));
    frame
}

fn widget<T>(child: &T) -> gtk4::Widget
where
    T: IsA<gtk4::Widget> + Clone + 'static,
{
    child.clone().upcast::<gtk4::Widget>()
}

fn notice(text: &str) -> gtk4::Label {
    let label = gtk4::Label::new(Some(text));
    label.set_wrap(true);
    label.set_xalign(0.0);
    label.add_css_class("dim-label");
    label
}

fn path_picker(title: &str, action: gtk4::FileChooserAction) -> (gtk4::Box, Rc<RefCell<String>>) {
    let selected = Rc::new(RefCell::new(String::new()));
    let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    row.set_hexpand(true);

    let button = gtk4::Button::with_label(title);
    let label = gtk4::Label::new(Some(tr("Aucun fichier sélectionné", "No file selected")));
    label.set_xalign(0.0);
    label.set_hexpand(true);
    label.add_css_class("dim-label");

    let selected_for_click = selected.clone();
    let label_for_click = label.clone();
    let title = title.to_string();
    button.connect_clicked(move |_| {
        let accept_label = match action {
            gtk4::FileChooserAction::Save => tr("Enregistrer", "Save"),
            gtk4::FileChooserAction::SelectFolder => tr("Choisir", "Choose"),
            _ => tr("Ouvrir", "Open"),
        };
        let dialog = gtk4::FileChooserNative::new(
            Some(&title),
            Option::<&gtk4::Window>::None,
            action,
            Some(accept_label),
            Some(tr("Annuler", "Cancel")),
        );
        let selected = selected_for_click.clone();
        let label = label_for_click.clone();
        dialog.connect_response(move |dialog, response| {
            if response == gtk4::ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        let path = path.display().to_string();
                        *selected.borrow_mut() = path.clone();
                        label.set_text(&path);
                    }
                }
            }
        });
        dialog.show();
    });

    row.append(&button);
    row.append(&label);
    (row, selected)
}

fn selected_path(path: &Rc<RefCell<String>>) -> Result<String, String> {
    let path = path.borrow().trim().to_string();
    if path.is_empty() {
        Err(tr(
            "aucun fichier ou dossier sélectionné",
            "no file or folder selected",
        )
        .into())
    } else {
        Ok(path)
    }
}

fn key_combo(secret: bool) -> gtk4::ComboBoxText {
    let combo = gtk4::ComboBoxText::new();
    combo.set_hexpand(true);
    populate_key_combo(&combo, secret);
    combo
}

fn certificate_combo() -> gtk4::ComboBoxText {
    let combo = gtk4::ComboBoxText::new();
    combo.set_hexpand(true);
    populate_certificate_combo(&combo);
    combo
}

fn trust_combo() -> gtk4::ComboBoxText {
    let combo = gtk4::ComboBoxText::new();
    combo.set_hexpand(true);
    combo.append(Some("unknown"), tr("Confiance inconnue", "Unknown Trust"));
    combo.append(
        Some("never"),
        tr("Ne jamais faire confiance", "Never Trust"),
    );
    combo.append(
        Some("marginal"),
        tr("Confiance marginale", "Marginal Trust"),
    );
    combo.append(Some("full"), tr("Confiance complète", "Full Trust"));
    combo.append(Some("ultimate"), tr("Confiance ultime", "Ultimate Trust"));
    combo.set_active(Some(0));
    combo
}

fn selected_id(combo: &gtk4::ComboBoxText) -> Result<String, String> {
    combo
        .active_id()
        .map(|id| id.to_string())
        .filter(|id| !id.trim().is_empty())
        .ok_or_else(|| {
            tr(
                "aucune clé ou certificat sélectionné",
                "no key or certificate selected",
            )
            .into()
        })
}

fn selected_key_id(selected_key: &Rc<RefCell<String>>) -> Result<String, String> {
    let key = selected_key.borrow().trim().to_string();
    if key.is_empty() {
        Err(tr(
            "aucune clé sélectionnée dans la liste",
            "no key selected in the list",
        )
        .into())
    } else {
        Ok(key)
    }
}

fn populate_key_list(
    list: &gtk4::ListBox,
    selected_key: &Rc<RefCell<String>>,
    selected_label: &gtk4::Label,
    secret: bool,
) {
    while let Some(child) = list.first_child() {
        list.remove(&child);
    }
    selected_key.borrow_mut().clear();
    selected_label.set_text(tr("Aucune clé sélectionnée", "No key selected"));

    let command = if secret {
        "--list-secret-keys"
    } else {
        "--list-keys"
    };
    match run_gpg(&["--with-colons", "--fingerprint", command]) {
        Ok(text) => {
            let identities = parse_identities(&text);
            if identities.is_empty() {
                list.append(&key_row(
                    "",
                    tr("Aucune clé OpenPGP trouvée", "No OpenPGP key found"),
                    "",
                ));
                return;
            }
            for (fingerprint, user_id) in &identities {
                list.append(&key_row(fingerprint, user_id, fingerprint));
            }
            if let Some((fingerprint, _)) = identities.first() {
                *selected_key.borrow_mut() = fingerprint.clone();
                selected_label.set_text(&format!(
                    "{}: {fingerprint}",
                    tr("Clé sélectionnée", "Selected key")
                ));
            }
            if let Some(row) = list.row_at_index(0) {
                list.select_row(Some(&row));
            }
        }
        Err(err) => list.append(&key_row(
            "",
            tr("Erreur GnuPG", "GnuPG Error"),
            short_status(&err),
        )),
    }
}

fn key_row(fingerprint: &str, title: &str, subtitle: &str) -> gtk4::ListBoxRow {
    let row = gtk4::ListBoxRow::new();
    row.set_widget_name(fingerprint);

    let box_ = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    box_.set_margin_top(8);
    box_.set_margin_bottom(8);
    box_.set_margin_start(10);
    box_.set_margin_end(10);

    let title = gtk4::Label::new(Some(title));
    title.set_xalign(0.0);
    title.add_css_class("heading");
    let subtitle = gtk4::Label::new(Some(subtitle));
    subtitle.set_xalign(0.0);
    subtitle.add_css_class("dim-label");
    subtitle.set_selectable(true);

    box_.append(&title);
    box_.append(&subtitle);
    row.set_child(Some(&box_));
    row
}

fn populate_key_combo(combo: &gtk4::ComboBoxText, secret: bool) {
    combo.remove_all();
    let command = if secret {
        "--list-secret-keys"
    } else {
        "--list-keys"
    };
    match run_gpg(&["--with-colons", "--fingerprint", command]) {
        Ok(text) => populate_identity_combo(
            combo,
            &text,
            tr("Aucune clé OpenPGP trouvée", "No OpenPGP key found"),
        ),
        Err(err) => combo.append(
            Some(""),
            &format!(
                "{}: {}",
                tr("Erreur GnuPG", "GnuPG Error"),
                short_status(&err)
            ),
        ),
    }
    combo.set_active(Some(0));
}

fn populate_certificate_combo(combo: &gtk4::ComboBoxText) {
    combo.remove_all();
    match run_gpgsm(&["--with-colons", "--list-keys"]) {
        Ok(text) => populate_identity_combo(
            combo,
            &text,
            tr(
                "Aucun certificat X.509 trouvé",
                "No X.509 certificate found",
            ),
        ),
        Err(err) => combo.append(
            Some(""),
            &format!(
                "{}: {}",
                tr("Erreur gpgsm", "gpgsm error"),
                short_status(&err)
            ),
        ),
    }
    combo.set_active(Some(0));
}

fn populate_identity_combo(combo: &gtk4::ComboBoxText, text: &str, empty_label: &str) {
    let identities = parse_identities(text);
    if identities.is_empty() {
        combo.append(Some(""), empty_label);
        return;
    }
    for (fingerprint, user_id) in identities {
        combo.append(Some(&fingerprint), &format!("{user_id}  [{fingerprint}]"));
    }
}

fn parse_identities(text: &str) -> Vec<(String, String)> {
    let mut pending_fpr: Option<String> = None;
    let mut identities = Vec::new();
    for line in text.lines() {
        let fields = line.split(':').collect::<Vec<_>>();
        match fields.first().copied() {
            Some("fpr") => pending_fpr = fields.get(9).map(|value| value.to_string()),
            Some("uid") => {
                if let Some(fpr) = pending_fpr.take() {
                    let uid = fields.get(9).copied().unwrap_or("Identité sans nom");
                    identities.push((fpr, uid.to_string()));
                }
            }
            _ => {}
        }
    }
    if let Some(fpr) = pending_fpr {
        identities.push((fpr.clone(), fpr));
    }
    identities
}

fn entry(placeholder: &str) -> gtk4::Entry {
    let entry = gtk4::Entry::new();
    entry.set_placeholder_text(Some(placeholder));
    entry.set_hexpand(true);
    entry
}

fn button(label: &str) -> gtk4::Button {
    let button = gtk4::Button::with_label(label);
    button.set_halign(gtk4::Align::Start);
    button
}

fn text_output(editable: bool) -> gtk4::TextView {
    let view = gtk4::TextView::new();
    view.set_editable(editable);
    view.set_monospace(true);
    view.set_vexpand(true);
    view.set_wrap_mode(gtk4::WrapMode::WordChar);
    view
}

fn scroll(child: &impl IsA<gtk4::Widget>, min_height: i32) -> gtk4::ScrolledWindow {
    gtk4::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .min_content_height(min_height)
        .child(child)
        .build()
}

fn split_text_views(
    left_title: &str,
    left: &gtk4::TextView,
    right_title: &str,
    right: &gtk4::TextView,
) -> gtk4::Paned {
    let paned = gtk4::Paned::new(gtk4::Orientation::Horizontal);
    paned.set_hexpand(true);
    paned.set_vexpand(true);
    paned.set_start_child(Some(&labeled_text(left_title, left)));
    paned.set_end_child(Some(&labeled_text(right_title, right)));
    paned.set_position(580);
    paned
}

fn labeled_text(title: &str, text: &gtk4::TextView) -> gtk4::Box {
    let box_ = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    box_.set_hexpand(true);
    box_.set_vexpand(true);
    let label = gtk4::Label::new(Some(title));
    label.set_xalign(0.0);
    label.add_css_class("heading");
    box_.append(&label);
    box_.append(&scroll(text, 360));
    box_
}

fn status_label() -> gtk4::Label {
    let label = gtk4::Label::new(Some(tr("Prêt", "Ready")));
    label.set_halign(gtk4::Align::Start);
    label.set_wrap(true);
    label
}

fn button_row(buttons: &[&impl IsA<gtk4::Widget>]) -> gtk4::Box {
    let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    row.set_halign(gtk4::Align::Start);
    row.set_hexpand(true);
    for button in buttons {
        row.append(*button);
    }
    row
}

fn refresh_keys(output: &gtk4::TextView, status: &gtk4::Label) {
    let result = run_gpg(&["--list-keys", "--keyid-format", "long", "--fingerprint"]);
    set_status(status, result.clone());
    if let Ok(text) = result {
        output.buffer().set_text(&text);
    }
}

fn connect_status<F>(button: &gtk4::Button, status: &gtk4::Label, action: F)
where
    F: Fn() -> Result<String, String> + 'static,
{
    let status = status.clone();
    button.connect_clicked(move |_| set_status(&status, action()));
}

fn connect_output<F>(
    button: &gtk4::Button,
    output: &gtk4::TextView,
    status: &gtk4::Label,
    action: F,
) where
    F: Fn() -> Result<String, String> + 'static,
{
    let output = output.clone();
    let status = status.clone();
    button.connect_clicked(move |_| {
        let result = action();
        set_status(&status, result.clone());
        match result {
            Ok(text) | Err(text) => output.buffer().set_text(&text),
        }
    });
}

fn connect_text_transform<F>(
    button: &gtk4::Button,
    input: &gtk4::TextView,
    output: &gtk4::TextView,
    status: &gtk4::Label,
    action: F,
) where
    F: Fn(&str) -> Result<String, String> + 'static,
{
    let input = input.clone();
    let output = output.clone();
    let status = status.clone();
    button.connect_clicked(move |_| {
        let buffer = input.buffer();
        let text = buffer
            .text(&buffer.start_iter(), &buffer.end_iter(), false)
            .to_string();
        let result = action(&text);
        set_status(&status, result.clone());
        match result {
            Ok(text) | Err(text) => output.buffer().set_text(&text),
        }
    });
}

fn set_status(label: &gtk4::Label, result: Result<String, String>) {
    match result {
        Ok(text) if text.trim().is_empty() => {
            label.set_text(tr("Opération terminée.", "Operation completed."))
        }
        Ok(text) => label.set_text(short_status(&text)),
        Err(err) => label.set_text(&format!(
            "{}: {}",
            tr("Erreur", "Error"),
            short_status(&err)
        )),
    }
}

fn short_status(text: &str) -> &str {
    text.lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or(tr("Opération terminée.", "Operation completed."))
}

fn run_gpg(args: &[&str]) -> Result<String, String> {
    run_program(
        "gpg",
        &args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>(),
    )
}

fn run_gpgsm(args: &[&str]) -> Result<String, String> {
    run_program(
        "gpgsm",
        &args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>(),
    )
}

fn run_gpg_with_keyserver(keyserver: &str, args: &[&str]) -> Result<String, String> {
    let mut full_args = Vec::new();
    if !keyserver.trim().is_empty() {
        full_args.push("--keyserver".to_string());
        full_args.push(keyserver.trim().to_string());
    }
    full_args.extend(args.iter().map(|arg| arg.to_string()));
    run_program("gpg", &full_args)
}

fn run_program(program: &str, args: &[String]) -> Result<String, String> {
    validate_command_args(args)?;
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|err| err.to_string())?;
    command_result(output)
}

fn run_program_with_input(program: &str, args: &[String], input: &str) -> Result<String, String> {
    validate_command_args(args)?;
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| err.to_string())?;
    let stdin = child
        .stdin
        .as_mut()
        .ok_or_else(|| tr("stdin indisponible", "stdin unavailable").to_string())?;
    stdin
        .write_all(input.as_bytes())
        .map_err(|err| err.to_string())?;
    let output = child.wait_with_output().map_err(|err| err.to_string())?;
    command_result(output)
}

fn run_program_to_file(
    program: &str,
    args: &[String],
    path: &str,
    mode: u32,
) -> Result<String, String> {
    let path = Path::new(path);
    reject_dangerous_path(path)?;
    let text = run_program(program, args)?;
    fs::write(path, text).map_err(|err| err.to_string())?;
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).map_err(|err| err.to_string())?;
    Ok(format!("Export écrit: {}", path.display()))
}

fn command_result(output: std::process::Output) -> Result<String, String> {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let text = format!("{stdout}{stderr}");
    if output.status.success() {
        Ok(text)
    } else {
        Err(text)
    }
}

fn validate_command_args(args: &[String]) -> Result<(), String> {
    for arg in args {
        if arg.contains('\0') {
            return Err(tr(
                "entrée invalide: caractère NUL interdit",
                "invalid input: NUL character is forbidden",
            )
            .into());
        }
    }
    Ok(())
}

fn split_list(value: &str) -> Vec<String> {
    value
        .split([',', ';', ' ', '\n', '\t'])
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn create_vault_dir(path: &str) -> Result<String, String> {
    let path = Path::new(path);
    reject_dangerous_path(path)?;
    fs::create_dir_all(path).map_err(|err| err.to_string())?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o700)).map_err(|err| err.to_string())?;
    Ok(format!("Dossier sécurisé créé: {}", path.display()))
}

fn reject_dangerous_path(path: &Path) -> Result<(), String> {
    if path.as_os_str().is_empty() {
        return Err(tr("chemin vide", "empty path").into());
    }
    if path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(tr(
            "les chemins avec '..' sont refusés",
            "paths containing '..' are refused",
        )
        .into());
    }
    Ok(())
}

fn command_exists(tool: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {tool} >/dev/null 2>&1"))
        .status()
        .is_ok_and(|status| status.success())
}

fn tr(fr: &'static str, en: &'static str) -> &'static str {
    if english_ui() {
        en
    } else {
        fr
    }
}

fn english_ui() -> bool {
    ENGLISH_UI.load(Ordering::Relaxed)
}

fn default_english_ui() -> bool {
    !system_locale()
        .map(|value| value.to_ascii_lowercase().starts_with("fr"))
        .unwrap_or(false)
}

fn system_locale() -> Option<String> {
    env::var("LC_ALL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            env::var("LC_MESSAGES")
                .ok()
                .filter(|value| !value.trim().is_empty())
        })
        .or_else(|| {
            env::var("LANG")
                .ok()
                .filter(|value| !value.trim().is_empty())
        })
}

fn language_button_label() -> &'static str {
    if english_ui() {
        "Français"
    } else {
        "English"
    }
}

#[cfg(test)]
mod tests {
    use super::parse_identities;

    #[test]
    fn parses_gpg_colon_fingerprint_and_uid() {
        let text = "\
pub:u:255:22:ABCDEF0123456789:1710000000:::u:::scESC:::::ed25519:::0:
fpr:::::::::0123456789ABCDEF0123456789ABCDEF01234567:
uid:u::::1710000000::HASH::Alice Example <alice@example.test>::::::::::0:
";

        let identities = parse_identities(text);

        assert_eq!(identities.len(), 1);
        assert_eq!(identities[0].0, "0123456789ABCDEF0123456789ABCDEF01234567");
        assert_eq!(identities[0].1, "Alice Example <alice@example.test>");
    }
}
