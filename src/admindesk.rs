//! AdminDesk (A.D.M.I.N): branding i blokady ustawień. Jedyny haczyk: `load_custom_client()`.
//! Rola z czasu kompilacji: ADMINDESK_ROLE=agent (domyślnie, u klienta) | konsola (technik).

use base::config::keys;
use hbb_common::config;

// Etap 2: nasz hbbs i jego klucz PUBLICZNY (id_ed25519.pub). Puste = jeszcze brak serwera.
const RENDEZVOUS_SERVER: &str = "";
const SERVER_KEY: &str = "";

pub fn is_konsola() -> bool {
    option_env!("ADMINDESK_ROLE") == Some("konsola")
}

pub fn apply() {
    let konsola = is_konsola();
    *config::APP_NAME.write().unwrap() =
        if konsola { "AdminDesk-Konsola" } else { "AdminDesk" }.to_owned();

    let mut hard = config::HARD_SETTINGS.write().unwrap();
    hard.insert("conn-type".to_owned(), if konsola { "outgoing" } else { "incoming" }.to_owned());
    if !konsola {
        hard.insert("disable-settings".to_owned(), "Y".to_owned());
    }
    drop(hard);

    let mut builtin = config::BUILTIN_SETTINGS.write().unwrap();
    for k in [
        keys::OPTION_HIDE_SERVER_SETTINGS,
        keys::OPTION_HIDE_NETWORK_SETTINGS,
        keys::OPTION_HIDE_PROXY_SETTINGS,
        keys::OPTION_HIDE_WEBSOCKET_SETTINGS,
        keys::OPTION_HIDE_HELP_CARDS,
        keys::OPTION_HIDE_POWERED_BY_ME,
        keys::OPTION_DISABLE_CHANGE_ID,
        // do czasu Control Plane: technik nadaje hasło per komputer `--password` (wymaga admina)
        keys::OPTION_ALLOW_COMMAND_LINE_SETTINGS_WHEN_SETTINGS_DISABLED,
    ] {
        builtin.insert(k.to_owned(), "Y".to_owned());
    }
    if !konsola {
        builtin.insert(keys::OPTION_HIDE_SECURITY_SETTINGS.to_owned(), "Y".to_owned());
    }
    drop(builtin);

    // OVERWRITE ma pierwszeństwo przed plikiem konfiguracji (Config::get_option)
    let mut overwrite = config::OVERWRITE_SETTINGS.write().unwrap();
    if !konsola {
        for k in [keys::OPTION_ENABLE_TUNNEL, keys::OPTION_ENABLE_TERMINAL, keys::OPTION_ENABLE_CAMERA] {
            overwrite.insert(k.to_owned(), "N".to_owned());
        }
    }
    if !RENDEZVOUS_SERVER.is_empty() {
        overwrite.insert(keys::OPTION_CUSTOM_RENDEZVOUS_SERVER.to_owned(), RENDEZVOUS_SERVER.to_owned());
        overwrite.insert(keys::OPTION_RELAY_SERVER.to_owned(), RENDEZVOUS_SERVER.to_owned());
        overwrite.insert(keys::OPTION_KEY.to_owned(), SERVER_KEY.to_owned());
    }
}
