use std::process::Command;

#[test]
fn subcommands_are_explicit_placeholders() {
    for command in ["record", "report", "serve"] {
        let output = Command::new(env!("CARGO_BIN_EXE_lpp-cli"))
            .arg(command)
            .output()
            .expect("CLI should launch");
        assert!(output.status.success());
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            format!("{command}: not implemented yet\n")
        );
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn help_describes_subcommands() {
    for flag in ["--help", "-h"] {
        let output = Command::new(env!("CARGO_BIN_EXE_lpp-cli"))
            .arg(flag)
            .output()
            .expect("CLI should launch");
        assert!(output.status.success());
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "Usage: lpp-cli <record|report|serve>\n"
        );
    }
}

#[test]
fn invalid_invocations_fail() {
    for args in [vec![], vec!["unknown"], vec!["record", "unexpected"]] {
        let output = Command::new(env!("CARGO_BIN_EXE_lpp-cli"))
            .args(args)
            .output()
            .expect("CLI should launch");
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("Usage:"));
    }
}
