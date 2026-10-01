// SPDX-License-Identifier: {{ license }}

//! Provides localization support for this crate.

use std::sync::LazyLock;

use i18n_embed::{
    DefaultLocalizer, LanguageLoader, Localizer,
    fluent::{FluentLanguageLoader, fluent_language_loader},
    unic_langid::LanguageIdentifier,
};
use icu_collator::{
    Collator, CollatorBorrowed, CollatorPreferences, options::CollatorOptions,
    preferences::CollationNumericOrdering,
};
use icu_provider::prelude::icu_locale_core::Locale;
use rust_embed::RustEmbed;

/// Applies the requested language(s) to requested translations from the `fl!()` macro.
pub fn init(requested_languages: &[LanguageIdentifier]) {
    if let Err(why) = localizer().select(requested_languages) {
        tracing::error!("error while loading fluent localizations: {why}");
    }
}

// Get the `Localizer` to be used for localizing this library.
#[must_use]
pub fn localizer() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub static LANGUAGE_LOADER: LazyLock<FluentLanguageLoader> = LazyLock::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    loader
        .load_fallback_language(&Localizations)
        .expect("Error while loading fallback language");

    loader
});

pub static LANGUAGE_SORTER: LazyLock<CollatorBorrowed> = LazyLock::new(|| {
    let create_collator = |locale: Locale| {
        let mut prefs = CollatorPreferences::from(locale);
        prefs.numeric_ordering = Some(CollationNumericOrdering::True);
        Collator::try_new(prefs, CollatorOptions::default()).ok()
    };

    Locale::try_from_str(&LANGUAGE_LOADER.current_language().to_string())
            .ok()
            .and_then(create_collator)
            .or_else(|| {
                Locale::try_from_str(&LANGUAGE_LOADER.fallback_language().to_string())
                    .ok()
                    .and_then(create_collator)
            })
            .unwrap_or_else(|| {
                let locale = Locale::try_from_str("en-US").expect("en-US is a valid BCP-47 tag");
                create_collator(locale)
                    .expect("Creating a collator from the system's current language, the fallback language, or American English should succeed")
            })
});

/// Request a localized string by ID from the i18n/ directory.
#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:tt)*) => {{
        i18n_embed_fl::fl!($crate::i18n::LANGUAGE_LOADER, $message_id, $($args)*)
    }};
}
