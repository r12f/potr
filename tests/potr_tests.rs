use potr::*;
use pretty_assertions::assert_eq;
use regex::Regex;

#[tokio::test]
async fn potr_should_translate_untranslated_messages() {
    let potr_config = PotrConfig::default();

    run_potr_test("untranslated", potr_config).await;
}

#[tokio::test]
async fn potr_should_translate_translated_messages() {
    let mut potr_config = PotrConfig::default();
    potr_config.skip_translated = false;

    run_potr_test("translated", potr_config).await;
}

#[tokio::test]
async fn potr_should_translate_messages_from_filtered_source_file() {
    let mut potr_config = PotrConfig::default();
    potr_config.source_regex = Some(Regex::new("Pen").unwrap());

    run_potr_test("source-filter", potr_config).await;
}

async fn run_potr_test(test_name: &str, mut potr_config: PotrConfig) {
    potr_config.po_file_path = format!("tests/data/{}-input.po", test_name);
    potr_config.output_file_path = format!("tests/data/{}-result.po", test_name);

    let translator_config = TranslatorConfig::default();

    let potr = Potr::new(potr_config.clone(), translator_config);
    potr.run().await.expect("Failed to run potr");

    let result =
        std::fs::read_to_string(&potr_config.output_file_path).expect("Failed to read result file");

    let expected_file_path = format!("tests/data/{}-expected.po", test_name);

    // If POTR_GENERATE_TEST_RESULT environment variable is set, we replace the expected file with the result file.
    if std::env::var("POTR_GENERATE_TEST_RESULT").is_ok() {
        std::fs::write(&expected_file_path, result).expect("Failed to write expected file");
    } else {
        let expected =
            std::fs::read_to_string(&expected_file_path).expect("Failed to read expected file");

        assert_eq!(result, expected);
    }
}
