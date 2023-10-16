use std::{
    collections::BTreeSet,
    env,
    path::PathBuf,
    process::{self, Command, ExitCode},
};

use vitasdk_sys_build_util::vita_headers_db::{
    missing_features_filter, missing_libs_filter, VitaDb,
};

fn vitasdk_sys_manifest() -> PathBuf {
    let cargo = env::var_os("CARGO");
    let output = Command::new(cargo.as_deref().unwrap_or("cargo".as_ref()))
        .args(["locate-project", "--message-format", "plain", "--workspace"])
        .stderr(process::Stdio::inherit())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "Could not cargo locate-project; perhaps running outside from workspace directory?"
    );
    String::from_utf8(output.stdout).unwrap().trim().into()
}

fn vita_headers_db_path() -> PathBuf {
    vitasdk_sys_manifest()
        .parent()
        .unwrap()
        .join("vita-headers/db")
}

fn main() -> ExitCode {
    env_logger::init();

    let args: TopLevelArgs = argh::from_env();
    match &args.subcommand {
        Subcommands::StubLibs(a) => stub_libs(a),
    }
}

#[derive(argh::FromArgs, PartialEq, Debug)]
/// Internal build utilities for vitasdk-sys crate
struct TopLevelArgs {
    #[argh(subcommand)]
    subcommand: Subcommands,
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Subcommands {
    StubLibs(SubCommandStubLibs),
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "stub-libs")]
/// Print stub lib names
struct SubCommandStubLibs {
    /// print only user stub libs, mutually exclusive with `--kernel`
    #[argh(switch, short = 'u')]
    user: bool,
    /// print only kernel stub libs, mutually exclusive with `--user`
    #[argh(switch, short = 'k')]
    kernel: bool,
    /// print only stub libs with conflicting symbols, mutually exclusive with `--with-conflicting`
    #[argh(switch, short = 'c')]
    conflicting: bool,
    /// include stub libs with conflicting symbols, mutually exclusive with `--conflicting`
    #[argh(switch)]
    with_conflicting: bool,
    /// print only undefined vitasdk-sys features
    #[argh(switch)]
    missing_features: bool,
    /// print stub libs as cargo features
    #[argh(switch)]
    as_features: bool,
    /// print only stub libs which do not exist in `$VITASDK/arm-vita-eabi/lib`
    #[argh(switch)]
    missing_libs: bool,
    /// fail if any stub lib is printed
    #[argh(switch)]
    fail_if_any: bool,
}

fn stub_libs(args: &SubCommandStubLibs) -> ExitCode {
    assert!(
        !(args.user & args.kernel),
        "`--kernel` and `--user` cli switches are mutually exclusive"
    );
    assert!(
        !(args.conflicting & args.with_conflicting),
        "`--conflicting` and `--with-conflicting` cli switches are mutually exclusive"
    );

    let mut db = VitaDb::load(&vita_headers_db_path());
    if args.conflicting {
        db = db.split_conflicting();
    } else if !args.with_conflicting {
        db.split_conflicting();
    }
    if args.user {
        db.split_kernel();
    } else if args.kernel {
        db = db.split_kernel();
    }

    let vitasdk_sys_manifest = vitasdk_sys_manifest();
    let stub_libs: BTreeSet<_> = {
        let mut missing_features_filter = args
            .missing_features
            .then(|| missing_features_filter(&vitasdk_sys_manifest));
        let mut missing_libs_filter = args.missing_libs.then(missing_libs_filter);

        db.stub_lib_names()
            .filter(|s| {
                missing_features_filter
                    .as_mut()
                    .map(|f| f(s))
                    .unwrap_or(true)
            })
            .filter(|s| missing_libs_filter.as_mut().map(|f| f(s)).unwrap_or(true))
            .collect()
    };

    if args.as_features {
        for stub_lib in &stub_libs {
            println!("{stub_lib} = []")
        }
    } else {
        for stub_lib in &stub_libs {
            println!("{stub_lib}")
        }
    }

    if !stub_libs.is_empty() && args.fail_if_any {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
