use std::{collections::BTreeSet, ffi::OsString};

use crate::{
    fallible,
    os::{Os, Target},
    Arch, DesktopEnv, Language, Platform, Result,
};

macro_rules! report_message {
    () => {
        "Please report this issue at https://github.com/ardaku/whoami/issues"
    };
}

const DEFAULT_USERNAME: &str = "Unknown";
const DEFAULT_HOSTNAME: &str = "LocalHost";

/// Get the CPU Architecture.
#[inline(always)]
pub fn arch() -> Arch {
    Target::arch(Os).expect(concat!("arch() failed.  ", report_message!()))
}

/// Get the user's username.
///
/// On unix-systems this differs from [`realname()`] most notably in that spaces
/// are not allowed in the username.
#[inline(always)]
pub fn username() -> String {
    fallible::username().unwrap_or_else(|_| DEFAULT_USERNAME.to_lowercase())
}

/// Get the user's username.
///
/// On unix-systems this differs from [`realname_os()`] most notably in that
/// spaces are not allowed in the username.
#[inline(always)]
pub fn username_os() -> OsString {
    fallible::username_os()
        .unwrap_or_else(|_| DEFAULT_USERNAME.to_lowercase().into())
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname() -> String {
    fallible::realname()
        .or_else(|_| fallible::username())
        .unwrap_or_else(|_| DEFAULT_USERNAME.to_owned())
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname_os() -> OsString {
    fallible::realname_os()
        .or_else(|_| fallible::username_os())
        .unwrap_or_else(|_| DEFAULT_USERNAME.to_owned().into())
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename() -> String {
    fallible::devicename()
        .or_else(|_| fallible::hostname())
        .unwrap_or_else(|_| DEFAULT_HOSTNAME.to_string())
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename_os() -> OsString {
    fallible::devicename_os()
        .or_else(|_| fallible::hostname().map(OsString::from))
        .unwrap_or_else(|_| DEFAULT_HOSTNAME.to_string().into())
}

/// Get the host device's hostname.
///
/// This method normalizes to lowercase.  Usually hostnames are
/// case-insensitive, but it's not a hard requirement.
///
/// FIXME: Document platform-specific character limitations
///
/// Use [`fallible::hostname()`] for case-sensitive hostname.
#[inline(always)]
#[deprecated(note = "use `fallible::hostname()` instead", since = "1.5.0")]
pub fn hostname() -> String {
    let mut hostname = fallible::hostname()
        .unwrap_or_else(|_| DEFAULT_HOSTNAME.to_lowercase());

    hostname.make_ascii_lowercase();
    hostname
}

/// Get the host device's hostname.
///
/// Usually hostnames are case-insensitive, but it's
/// not a hard requirement.
///
/// Use [`fallible::hostname()`] for case-sensitive hostname.
#[inline(always)]
#[deprecated(note = "use `fallible::hostname()` instead", since = "1.5.0")]
pub fn hostname_os() -> OsString {
    #[allow(deprecated)]
    hostname().into()
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn distro() -> String {
    fallible::distro().unwrap_or_else(|_| format!("Unknown {}", platform()))
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
#[deprecated(note = "use `distro()` instead", since = "1.5.0")]
pub fn distro_os() -> OsString {
    fallible::distro()
        .map(OsString::from)
        .unwrap_or_else(|_| format!("Unknown {}", platform()).into())
}

/// Get the desktop environment.
///
/// Example: "gnome" or "windows"
#[inline(always)]
pub fn desktop_env() -> DesktopEnv {
    Target::desktop_env(Os)
}

/// Get the platform.
#[inline(always)]
pub fn platform() -> Platform {
    Target::platform(Os)
}

/// Get the user's preferred language(s).
///
/// Returned as iterator of two letter language codes (lowercase), optionally
/// followed by a dash (-) and a two letter country code (uppercase).  The most
/// preferred language is returned first, followed by next preferred, and so on.
#[inline(always)]
#[deprecated(note = "use `langs()` instead", since = "1.5.0")]
pub fn lang() -> impl Iterator<Item = String> {
    let langs_vec = if let Ok(langs) = langs(Some(false)) {
        langs
            .map(|lang| lang.to_string().replace('/', "-"))
            .collect()
    } else {
        ["en-US".to_string()].to_vec()
    };

    langs_vec.into_iter()
}

/// Get the user's preferred language(s).
///
/// Returned as iterator of [`Language`]s.  The most preferred language is
/// returned first, followed by next preferred, and so on.  Unrecognized
/// languages may either return an error or be skipped.
#[inline(always)]
pub fn langs(fallback: Option<bool>) -> Result<impl Iterator<Item = Language>> {

    let langs = Target::langs(Os)?;

    let separator = if langs.contains(':') { ':' } else if langs.contains(';') { ';' } else { ',' };

    let mut lang_vec: Vec<Language> = langs
        .split(separator)
        .filter_map(|lang| {
            let lang = lang
                .split_terminator('.')
                .next()
                .unwrap_or_default()
                .replace(['_', '-'], "/");

            if lang == "C" {
                return None;
            }

            Some(Language::__(Box::new(lang)))
        })
        .collect();
    let fallback = fallback.unwrap_or(true);
    if fallback {
        let mut short_langs = BTreeSet::new();

    for lang in &lang_vec {
        if let Some(short) = lang.to_string().split('/').next() {
            short_langs.insert(short.to_string());
        }
    }

    for shorted in short_langs {
        if !lang_vec.iter().any(|lang| lang.to_string() == shorted) {
            lang_vec.push(Language::__(Box::new(shorted)));
        }
    }
    }
    

    Ok(lang_vec.into_iter())
}
