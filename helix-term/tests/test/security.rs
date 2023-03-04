use helix_view::{doc, editor::SecurityConfig};

use super::*;

fn security_test_config(security: SecurityConfig) -> Config {
    Config {
        editor: helix_view::editor::Config {
            security,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn scratch_buffer_trusted() -> anyhow::Result<()> {
    let config = security_test_config(SecurityConfig {
        ..Default::default()
    });
    let mut app = helpers::AppBuilder::new().with_config(config).build()?;

    test_key_sequence(
        &mut app,
        None,
        Some(&|app| {
            let doc = doc!(app.editor);
            assert!(doc.is_trusted());
        }),
        false,
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn scratch_buffer_restricted() -> anyhow::Result<()> {
    let config = security_test_config(SecurityConfig {
        trust_scratch_buffer: false,
        ..Default::default()
    });
    let mut app = helpers::AppBuilder::new().with_config(config).build()?;

    test_key_sequence(
        &mut app,
        None,
        Some(&|app| {
            let doc = doc!(app.editor);
            assert!(!doc.is_trusted());
        }),
        false,
    )
    .await?;

    Ok(())
}

// Document should be restricted by default
#[tokio::test(flavor = "multi_thread")]
async fn document_enable_trust() -> anyhow::Result<()> {
    let file = helpers::new_readonly_tempfile()?;
    let config = security_test_config(SecurityConfig {
        ..Default::default()
    });
    let mut app = helpers::AppBuilder::new()
        .with_config(config)
        .with_file(file.path(), None)
        .build()?;

    test_key_sequences(
        &mut app,
        vec![
            (
                None,
                Some(&|app| {
                    let doc = doc!(app.editor);
                    assert!(!doc.is_trusted());
                }),
            ),
            (
                Some(":trust true<ret>"),
                Some(&|app| {
                    let doc = doc!(app.editor);
                    assert!(doc.is_trusted());
                }),
            ),
        ],
        false,
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn document_disable_trust() -> anyhow::Result<()> {
    let file = helpers::new_readonly_tempfile()?;
    let directory = format!("{}", file.path().parent().unwrap().display());
    let config = security_test_config(SecurityConfig {
        trusted: vec![directory],
        ..Default::default()
    });
    let mut app = helpers::AppBuilder::new()
        .with_config(config)
        .with_file(file.path(), None)
        .build()?;

    test_key_sequences(
        &mut app,
        vec![
            (
                None,
                Some(&|app| {
                    let doc = doc!(app.editor);
                    assert!(doc.is_trusted());
                }),
            ),
            (
                Some(":trust false<ret>"),
                Some(&|app| {
                    let doc = doc!(app.editor);
                    assert!(!doc.is_trusted());
                }),
            ),
        ],
        false,
    )
    .await?;

    Ok(())
}
