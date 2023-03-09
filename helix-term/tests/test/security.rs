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
            assert!(doc.is_trusted(), "scratch buffer trusted by default");
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
            assert!(!doc.is_trusted(), "scratch buffer restricted by config");
        }),
        false,
    )
    .await?;

    Ok(())
}

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
                    assert!(!doc.is_trusted(), "document restricted on open");
                }),
            ),
            (
                Some(":trust true<ret>"),
                Some(&|app| {
                    let doc = doc!(app.editor);
                    assert!(doc.is_trusted(), "command should enable trust");
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
    let directory = file.path().parent().unwrap().to_path_buf();
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
                    assert!(doc.is_trusted(), "document trusted on open");
                }),
            ),
            (
                Some(":trust false<ret>"),
                Some(&|app| {
                    let doc = doc!(app.editor);
                    assert!(!doc.is_trusted(), "command should disable trust");
                }),
            ),
        ],
        false,
    )
    .await?;

    Ok(())
}
