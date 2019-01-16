// This file was generated by gir (https://github.com/gtk-rs/gir @ 2f0a317)
// from gir-files (https://github.com/gtk-rs/gir-files @ b215ee8)
// DO NOT EDIT

extern crate gmime_sys;
extern crate shell_words;
extern crate tempdir;
use std::env;
use std::error::Error;
use std::path::Path;
use std::mem::{align_of, size_of};
use std::process::Command;
use std::str;
use gmime_sys::*;

static PACKAGES: &[&str] = &["gmime-3.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn define<'a, V: Into<Option<&'a str>>>(&mut self, var: &str, val: V) {
        let arg = match val.into() {
            None => format!("-D{}", var),
            Some(val) => format!("-D{}={}", var, val),
        };
        self.args.push(arg);
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}",
                               &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let mut cmd = Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}",
                           &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
    /// Number of tests that failed to compile.
    failed_to_compile: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn record_failed_to_compile(&mut self) {
        self.failed += 1;
        self.failed_to_compile += 1;
    }
    fn summary(&self) -> String {
        format!(
            "{} passed; {} failed (compilation errors: {})",
            self.passed,
            self.failed,
            self.failed_to_compile)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let tmpdir = tempdir::TempDir::new("abi").expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!("1",
               get_c_value(tmpdir.path(), &cc, "1").expect("C constant"),
               "failed to obtain correct constant value for 1");

    let mut results : Results = Default::default();
    for (i, &(name, rust_value)) in RUST_CONSTANTS.iter().enumerate() {
        match get_c_value(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(ref c_value) => {
                if rust_value == c_value {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_value, c_value);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("constants ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let tmpdir = tempdir::TempDir::new("abi").expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(Layout {size: 1, alignment: 1},
               get_c_layout(tmpdir.path(), &cc, "char").expect("C layout"),
               "failed to obtain correct layout for char type");

    let mut results : Results = Default::default();
    for (i, &(name, rust_layout)) in RUST_LAYOUTS.iter().enumerate() {
        match get_c_layout(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(c_layout) => {
                if rust_layout == c_layout {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_layout, &c_layout);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("layout    ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

fn get_c_layout(dir: &Path, cc: &Compiler, name: &str) -> Result<Layout, Box<Error>> {
    let exe = dir.join("layout");
    let mut cc = cc.clone();
    cc.define("ABI_TYPE_NAME", name);
    cc.compile(Path::new("tests/layout.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    let stdout = str::from_utf8(&output.stdout)?;
    let mut words = stdout.trim().split_whitespace();
    let size = words.next().unwrap().parse().unwrap();
    let alignment = words.next().unwrap().parse().unwrap();
    Ok(Layout {size, alignment})
}

fn get_c_value(dir: &Path, cc: &Compiler, name: &str) -> Result<String, Box<Error>> {
    let exe = dir.join("constant");
    let mut cc = cc.clone();
    cc.define("ABI_CONSTANT_NAME", name);
    cc.compile(Path::new("tests/constant.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    let output = str::from_utf8(&output.stdout)?.trim();
    if !output.starts_with("###gir test###") ||
       !output.ends_with("###gir test###") {
        return Err(format!("command {:?} return invalid output, {:?}",
                           &abi_cmd, &output).into());
    }

    Ok(String::from(&output[14..(output.len() - 14)]))
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    ("GMimeAddressType", Layout {size: size_of::<GMimeAddressType>(), alignment: align_of::<GMimeAddressType>()}),
    ("GMimeApplicationPkcs7Mime", Layout {size: size_of::<GMimeApplicationPkcs7Mime>(), alignment: align_of::<GMimeApplicationPkcs7Mime>()}),
    ("GMimeApplicationPkcs7MimeClass", Layout {size: size_of::<GMimeApplicationPkcs7MimeClass>(), alignment: align_of::<GMimeApplicationPkcs7MimeClass>()}),
    ("GMimeAutocryptHeader", Layout {size: size_of::<GMimeAutocryptHeader>(), alignment: align_of::<GMimeAutocryptHeader>()}),
    ("GMimeAutocryptHeaderClass", Layout {size: size_of::<GMimeAutocryptHeaderClass>(), alignment: align_of::<GMimeAutocryptHeaderClass>()}),
    ("GMimeAutocryptHeaderList", Layout {size: size_of::<GMimeAutocryptHeaderList>(), alignment: align_of::<GMimeAutocryptHeaderList>()}),
    ("GMimeAutocryptHeaderListClass", Layout {size: size_of::<GMimeAutocryptHeaderListClass>(), alignment: align_of::<GMimeAutocryptHeaderListClass>()}),
    ("GMimeAutocryptPreferEncrypt", Layout {size: size_of::<GMimeAutocryptPreferEncrypt>(), alignment: align_of::<GMimeAutocryptPreferEncrypt>()}),
    ("GMimeCertificate", Layout {size: size_of::<GMimeCertificate>(), alignment: align_of::<GMimeCertificate>()}),
    ("GMimeCertificateClass", Layout {size: size_of::<GMimeCertificateClass>(), alignment: align_of::<GMimeCertificateClass>()}),
    ("GMimeCertificateList", Layout {size: size_of::<GMimeCertificateList>(), alignment: align_of::<GMimeCertificateList>()}),
    ("GMimeCertificateListClass", Layout {size: size_of::<GMimeCertificateListClass>(), alignment: align_of::<GMimeCertificateListClass>()}),
    ("GMimeCharset", Layout {size: size_of::<GMimeCharset>(), alignment: align_of::<GMimeCharset>()}),
    ("GMimeCipherAlgo", Layout {size: size_of::<GMimeCipherAlgo>(), alignment: align_of::<GMimeCipherAlgo>()}),
    ("GMimeContentDisposition", Layout {size: size_of::<GMimeContentDisposition>(), alignment: align_of::<GMimeContentDisposition>()}),
    ("GMimeContentDispositionClass", Layout {size: size_of::<GMimeContentDispositionClass>(), alignment: align_of::<GMimeContentDispositionClass>()}),
    ("GMimeContentEncoding", Layout {size: size_of::<GMimeContentEncoding>(), alignment: align_of::<GMimeContentEncoding>()}),
    ("GMimeContentType", Layout {size: size_of::<GMimeContentType>(), alignment: align_of::<GMimeContentType>()}),
    ("GMimeContentTypeClass", Layout {size: size_of::<GMimeContentTypeClass>(), alignment: align_of::<GMimeContentTypeClass>()}),
    ("GMimeCryptoContext", Layout {size: size_of::<GMimeCryptoContext>(), alignment: align_of::<GMimeCryptoContext>()}),
    ("GMimeCryptoContextClass", Layout {size: size_of::<GMimeCryptoContextClass>(), alignment: align_of::<GMimeCryptoContextClass>()}),
    ("GMimeDataWrapper", Layout {size: size_of::<GMimeDataWrapper>(), alignment: align_of::<GMimeDataWrapper>()}),
    ("GMimeDataWrapperClass", Layout {size: size_of::<GMimeDataWrapperClass>(), alignment: align_of::<GMimeDataWrapperClass>()}),
    ("GMimeDecryptFlags", Layout {size: size_of::<GMimeDecryptFlags>(), alignment: align_of::<GMimeDecryptFlags>()}),
    ("GMimeDecryptResult", Layout {size: size_of::<GMimeDecryptResult>(), alignment: align_of::<GMimeDecryptResult>()}),
    ("GMimeDecryptResultClass", Layout {size: size_of::<GMimeDecryptResultClass>(), alignment: align_of::<GMimeDecryptResultClass>()}),
    ("GMimeDigestAlgo", Layout {size: size_of::<GMimeDigestAlgo>(), alignment: align_of::<GMimeDigestAlgo>()}),
    ("GMimeEncoding", Layout {size: size_of::<GMimeEncoding>(), alignment: align_of::<GMimeEncoding>()}),
    ("GMimeEncodingConstraint", Layout {size: size_of::<GMimeEncodingConstraint>(), alignment: align_of::<GMimeEncodingConstraint>()}),
    ("GMimeEncryptFlags", Layout {size: size_of::<GMimeEncryptFlags>(), alignment: align_of::<GMimeEncryptFlags>()}),
    ("GMimeFilter", Layout {size: size_of::<GMimeFilter>(), alignment: align_of::<GMimeFilter>()}),
    ("GMimeFilterBasic", Layout {size: size_of::<GMimeFilterBasic>(), alignment: align_of::<GMimeFilterBasic>()}),
    ("GMimeFilterBasicClass", Layout {size: size_of::<GMimeFilterBasicClass>(), alignment: align_of::<GMimeFilterBasicClass>()}),
    ("GMimeFilterBestClass", Layout {size: size_of::<GMimeFilterBestClass>(), alignment: align_of::<GMimeFilterBestClass>()}),
    ("GMimeFilterBestFlags", Layout {size: size_of::<GMimeFilterBestFlags>(), alignment: align_of::<GMimeFilterBestFlags>()}),
    ("GMimeFilterCharset", Layout {size: size_of::<GMimeFilterCharset>(), alignment: align_of::<GMimeFilterCharset>()}),
    ("GMimeFilterCharsetClass", Layout {size: size_of::<GMimeFilterCharsetClass>(), alignment: align_of::<GMimeFilterCharsetClass>()}),
    ("GMimeFilterChecksum", Layout {size: size_of::<GMimeFilterChecksum>(), alignment: align_of::<GMimeFilterChecksum>()}),
    ("GMimeFilterChecksumClass", Layout {size: size_of::<GMimeFilterChecksumClass>(), alignment: align_of::<GMimeFilterChecksumClass>()}),
    ("GMimeFilterClass", Layout {size: size_of::<GMimeFilterClass>(), alignment: align_of::<GMimeFilterClass>()}),
    ("GMimeFilterDos2Unix", Layout {size: size_of::<GMimeFilterDos2Unix>(), alignment: align_of::<GMimeFilterDos2Unix>()}),
    ("GMimeFilterDos2UnixClass", Layout {size: size_of::<GMimeFilterDos2UnixClass>(), alignment: align_of::<GMimeFilterDos2UnixClass>()}),
    ("GMimeFilterEnriched", Layout {size: size_of::<GMimeFilterEnriched>(), alignment: align_of::<GMimeFilterEnriched>()}),
    ("GMimeFilterEnrichedClass", Layout {size: size_of::<GMimeFilterEnrichedClass>(), alignment: align_of::<GMimeFilterEnrichedClass>()}),
    ("GMimeFilterFrom", Layout {size: size_of::<GMimeFilterFrom>(), alignment: align_of::<GMimeFilterFrom>()}),
    ("GMimeFilterFromClass", Layout {size: size_of::<GMimeFilterFromClass>(), alignment: align_of::<GMimeFilterFromClass>()}),
    ("GMimeFilterFromMode", Layout {size: size_of::<GMimeFilterFromMode>(), alignment: align_of::<GMimeFilterFromMode>()}),
    ("GMimeFilterGZip", Layout {size: size_of::<GMimeFilterGZip>(), alignment: align_of::<GMimeFilterGZip>()}),
    ("GMimeFilterGZipClass", Layout {size: size_of::<GMimeFilterGZipClass>(), alignment: align_of::<GMimeFilterGZipClass>()}),
    ("GMimeFilterGZipMode", Layout {size: size_of::<GMimeFilterGZipMode>(), alignment: align_of::<GMimeFilterGZipMode>()}),
    ("GMimeFilterHTMLClass", Layout {size: size_of::<GMimeFilterHTMLClass>(), alignment: align_of::<GMimeFilterHTMLClass>()}),
    ("GMimeFilterOpenPGP", Layout {size: size_of::<GMimeFilterOpenPGP>(), alignment: align_of::<GMimeFilterOpenPGP>()}),
    ("GMimeFilterOpenPGPClass", Layout {size: size_of::<GMimeFilterOpenPGPClass>(), alignment: align_of::<GMimeFilterOpenPGPClass>()}),
    ("GMimeFilterSmtpData", Layout {size: size_of::<GMimeFilterSmtpData>(), alignment: align_of::<GMimeFilterSmtpData>()}),
    ("GMimeFilterSmtpDataClass", Layout {size: size_of::<GMimeFilterSmtpDataClass>(), alignment: align_of::<GMimeFilterSmtpDataClass>()}),
    ("GMimeFilterStrip", Layout {size: size_of::<GMimeFilterStrip>(), alignment: align_of::<GMimeFilterStrip>()}),
    ("GMimeFilterStripClass", Layout {size: size_of::<GMimeFilterStripClass>(), alignment: align_of::<GMimeFilterStripClass>()}),
    ("GMimeFilterUnix2Dos", Layout {size: size_of::<GMimeFilterUnix2Dos>(), alignment: align_of::<GMimeFilterUnix2Dos>()}),
    ("GMimeFilterUnix2DosClass", Layout {size: size_of::<GMimeFilterUnix2DosClass>(), alignment: align_of::<GMimeFilterUnix2DosClass>()}),
    ("GMimeFilterWindows", Layout {size: size_of::<GMimeFilterWindows>(), alignment: align_of::<GMimeFilterWindows>()}),
    ("GMimeFilterWindowsClass", Layout {size: size_of::<GMimeFilterWindowsClass>(), alignment: align_of::<GMimeFilterWindowsClass>()}),
    ("GMimeFilterYenc", Layout {size: size_of::<GMimeFilterYenc>(), alignment: align_of::<GMimeFilterYenc>()}),
    ("GMimeFilterYencClass", Layout {size: size_of::<GMimeFilterYencClass>(), alignment: align_of::<GMimeFilterYencClass>()}),
    ("GMimeFormat", Layout {size: size_of::<GMimeFormat>(), alignment: align_of::<GMimeFormat>()}),
    ("GMimeHeader", Layout {size: size_of::<GMimeHeader>(), alignment: align_of::<GMimeHeader>()}),
    ("GMimeHeaderClass", Layout {size: size_of::<GMimeHeaderClass>(), alignment: align_of::<GMimeHeaderClass>()}),
    ("GMimeHeaderList", Layout {size: size_of::<GMimeHeaderList>(), alignment: align_of::<GMimeHeaderList>()}),
    ("GMimeHeaderListClass", Layout {size: size_of::<GMimeHeaderListClass>(), alignment: align_of::<GMimeHeaderListClass>()}),
    ("GMimeMessage", Layout {size: size_of::<GMimeMessage>(), alignment: align_of::<GMimeMessage>()}),
    ("GMimeMessageClass", Layout {size: size_of::<GMimeMessageClass>(), alignment: align_of::<GMimeMessageClass>()}),
    ("GMimeMessagePart", Layout {size: size_of::<GMimeMessagePart>(), alignment: align_of::<GMimeMessagePart>()}),
    ("GMimeMessagePartClass", Layout {size: size_of::<GMimeMessagePartClass>(), alignment: align_of::<GMimeMessagePartClass>()}),
    ("GMimeMessagePartial", Layout {size: size_of::<GMimeMessagePartial>(), alignment: align_of::<GMimeMessagePartial>()}),
    ("GMimeMessagePartialClass", Layout {size: size_of::<GMimeMessagePartialClass>(), alignment: align_of::<GMimeMessagePartialClass>()}),
    ("GMimeMultipart", Layout {size: size_of::<GMimeMultipart>(), alignment: align_of::<GMimeMultipart>()}),
    ("GMimeMultipartClass", Layout {size: size_of::<GMimeMultipartClass>(), alignment: align_of::<GMimeMultipartClass>()}),
    ("GMimeMultipartEncrypted", Layout {size: size_of::<GMimeMultipartEncrypted>(), alignment: align_of::<GMimeMultipartEncrypted>()}),
    ("GMimeMultipartEncryptedClass", Layout {size: size_of::<GMimeMultipartEncryptedClass>(), alignment: align_of::<GMimeMultipartEncryptedClass>()}),
    ("GMimeMultipartSigned", Layout {size: size_of::<GMimeMultipartSigned>(), alignment: align_of::<GMimeMultipartSigned>()}),
    ("GMimeMultipartSignedClass", Layout {size: size_of::<GMimeMultipartSignedClass>(), alignment: align_of::<GMimeMultipartSignedClass>()}),
    ("GMimeNewLineFormat", Layout {size: size_of::<GMimeNewLineFormat>(), alignment: align_of::<GMimeNewLineFormat>()}),
    ("GMimeObject", Layout {size: size_of::<GMimeObject>(), alignment: align_of::<GMimeObject>()}),
    ("GMimeObjectClass", Layout {size: size_of::<GMimeObjectClass>(), alignment: align_of::<GMimeObjectClass>()}),
    ("GMimeOpenPGPData", Layout {size: size_of::<GMimeOpenPGPData>(), alignment: align_of::<GMimeOpenPGPData>()}),
    ("GMimeOpenPGPMarker", Layout {size: size_of::<GMimeOpenPGPMarker>(), alignment: align_of::<GMimeOpenPGPMarker>()}),
    ("GMimeOpenPGPState", Layout {size: size_of::<GMimeOpenPGPState>(), alignment: align_of::<GMimeOpenPGPState>()}),
    ("GMimeParam", Layout {size: size_of::<GMimeParam>(), alignment: align_of::<GMimeParam>()}),
    ("GMimeParamClass", Layout {size: size_of::<GMimeParamClass>(), alignment: align_of::<GMimeParamClass>()}),
    ("GMimeParamEncodingMethod", Layout {size: size_of::<GMimeParamEncodingMethod>(), alignment: align_of::<GMimeParamEncodingMethod>()}),
    ("GMimeParamList", Layout {size: size_of::<GMimeParamList>(), alignment: align_of::<GMimeParamList>()}),
    ("GMimeParamListClass", Layout {size: size_of::<GMimeParamListClass>(), alignment: align_of::<GMimeParamListClass>()}),
    ("GMimeParser", Layout {size: size_of::<GMimeParser>(), alignment: align_of::<GMimeParser>()}),
    ("GMimeParserClass", Layout {size: size_of::<GMimeParserClass>(), alignment: align_of::<GMimeParserClass>()}),
    ("GMimeParserWarning", Layout {size: size_of::<GMimeParserWarning>(), alignment: align_of::<GMimeParserWarning>()}),
    ("GMimePart", Layout {size: size_of::<GMimePart>(), alignment: align_of::<GMimePart>()}),
    ("GMimePartClass", Layout {size: size_of::<GMimePartClass>(), alignment: align_of::<GMimePartClass>()}),
    ("GMimePubKeyAlgo", Layout {size: size_of::<GMimePubKeyAlgo>(), alignment: align_of::<GMimePubKeyAlgo>()}),
    ("GMimeReferences", Layout {size: size_of::<GMimeReferences>(), alignment: align_of::<GMimeReferences>()}),
    ("GMimeRfcComplianceMode", Layout {size: size_of::<GMimeRfcComplianceMode>(), alignment: align_of::<GMimeRfcComplianceMode>()}),
    ("GMimeSecureMimeType", Layout {size: size_of::<GMimeSecureMimeType>(), alignment: align_of::<GMimeSecureMimeType>()}),
    ("GMimeSeekWhence", Layout {size: size_of::<GMimeSeekWhence>(), alignment: align_of::<GMimeSeekWhence>()}),
    ("GMimeSignature", Layout {size: size_of::<GMimeSignature>(), alignment: align_of::<GMimeSignature>()}),
    ("GMimeSignatureClass", Layout {size: size_of::<GMimeSignatureClass>(), alignment: align_of::<GMimeSignatureClass>()}),
    ("GMimeSignatureList", Layout {size: size_of::<GMimeSignatureList>(), alignment: align_of::<GMimeSignatureList>()}),
    ("GMimeSignatureListClass", Layout {size: size_of::<GMimeSignatureListClass>(), alignment: align_of::<GMimeSignatureListClass>()}),
    ("GMimeSignatureStatus", Layout {size: size_of::<GMimeSignatureStatus>(), alignment: align_of::<GMimeSignatureStatus>()}),
    ("GMimeStream", Layout {size: size_of::<GMimeStream>(), alignment: align_of::<GMimeStream>()}),
    ("GMimeStreamBuffer", Layout {size: size_of::<GMimeStreamBuffer>(), alignment: align_of::<GMimeStreamBuffer>()}),
    ("GMimeStreamBufferClass", Layout {size: size_of::<GMimeStreamBufferClass>(), alignment: align_of::<GMimeStreamBufferClass>()}),
    ("GMimeStreamBufferMode", Layout {size: size_of::<GMimeStreamBufferMode>(), alignment: align_of::<GMimeStreamBufferMode>()}),
    ("GMimeStreamCat", Layout {size: size_of::<GMimeStreamCat>(), alignment: align_of::<GMimeStreamCat>()}),
    ("GMimeStreamCatClass", Layout {size: size_of::<GMimeStreamCatClass>(), alignment: align_of::<GMimeStreamCatClass>()}),
    ("GMimeStreamClass", Layout {size: size_of::<GMimeStreamClass>(), alignment: align_of::<GMimeStreamClass>()}),
    ("GMimeStreamFile", Layout {size: size_of::<GMimeStreamFile>(), alignment: align_of::<GMimeStreamFile>()}),
    ("GMimeStreamFileClass", Layout {size: size_of::<GMimeStreamFileClass>(), alignment: align_of::<GMimeStreamFileClass>()}),
    ("GMimeStreamFilter", Layout {size: size_of::<GMimeStreamFilter>(), alignment: align_of::<GMimeStreamFilter>()}),
    ("GMimeStreamFilterClass", Layout {size: size_of::<GMimeStreamFilterClass>(), alignment: align_of::<GMimeStreamFilterClass>()}),
    ("GMimeStreamFs", Layout {size: size_of::<GMimeStreamFs>(), alignment: align_of::<GMimeStreamFs>()}),
    ("GMimeStreamFsClass", Layout {size: size_of::<GMimeStreamFsClass>(), alignment: align_of::<GMimeStreamFsClass>()}),
    ("GMimeStreamGIO", Layout {size: size_of::<GMimeStreamGIO>(), alignment: align_of::<GMimeStreamGIO>()}),
    ("GMimeStreamGIOClass", Layout {size: size_of::<GMimeStreamGIOClass>(), alignment: align_of::<GMimeStreamGIOClass>()}),
    ("GMimeStreamIOVector", Layout {size: size_of::<GMimeStreamIOVector>(), alignment: align_of::<GMimeStreamIOVector>()}),
    ("GMimeStreamMem", Layout {size: size_of::<GMimeStreamMem>(), alignment: align_of::<GMimeStreamMem>()}),
    ("GMimeStreamMemClass", Layout {size: size_of::<GMimeStreamMemClass>(), alignment: align_of::<GMimeStreamMemClass>()}),
    ("GMimeStreamMmap", Layout {size: size_of::<GMimeStreamMmap>(), alignment: align_of::<GMimeStreamMmap>()}),
    ("GMimeStreamMmapClass", Layout {size: size_of::<GMimeStreamMmapClass>(), alignment: align_of::<GMimeStreamMmapClass>()}),
    ("GMimeStreamNull", Layout {size: size_of::<GMimeStreamNull>(), alignment: align_of::<GMimeStreamNull>()}),
    ("GMimeStreamNullClass", Layout {size: size_of::<GMimeStreamNullClass>(), alignment: align_of::<GMimeStreamNullClass>()}),
    ("GMimeStreamPipe", Layout {size: size_of::<GMimeStreamPipe>(), alignment: align_of::<GMimeStreamPipe>()}),
    ("GMimeStreamPipeClass", Layout {size: size_of::<GMimeStreamPipeClass>(), alignment: align_of::<GMimeStreamPipeClass>()}),
    ("GMimeTextPart", Layout {size: size_of::<GMimeTextPart>(), alignment: align_of::<GMimeTextPart>()}),
    ("GMimeTextPartClass", Layout {size: size_of::<GMimeTextPartClass>(), alignment: align_of::<GMimeTextPartClass>()}),
    ("GMimeTrust", Layout {size: size_of::<GMimeTrust>(), alignment: align_of::<GMimeTrust>()}),
    ("GMimeValidity", Layout {size: size_of::<GMimeValidity>(), alignment: align_of::<GMimeValidity>()}),
    ("GMimeVerifyFlags", Layout {size: size_of::<GMimeVerifyFlags>(), alignment: align_of::<GMimeVerifyFlags>()}),
    ("InternetAddress", Layout {size: size_of::<InternetAddress>(), alignment: align_of::<InternetAddress>()}),
    ("InternetAddressClass", Layout {size: size_of::<InternetAddressClass>(), alignment: align_of::<InternetAddressClass>()}),
    ("InternetAddressGroup", Layout {size: size_of::<InternetAddressGroup>(), alignment: align_of::<InternetAddressGroup>()}),
    ("InternetAddressGroupClass", Layout {size: size_of::<InternetAddressGroupClass>(), alignment: align_of::<InternetAddressGroupClass>()}),
    ("InternetAddressList", Layout {size: size_of::<InternetAddressList>(), alignment: align_of::<InternetAddressList>()}),
    ("InternetAddressListClass", Layout {size: size_of::<InternetAddressListClass>(), alignment: align_of::<InternetAddressListClass>()}),
    ("InternetAddressMailbox", Layout {size: size_of::<InternetAddressMailbox>(), alignment: align_of::<InternetAddressMailbox>()}),
    ("InternetAddressMailboxClass", Layout {size: size_of::<InternetAddressMailboxClass>(), alignment: align_of::<InternetAddressMailboxClass>()}),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) GMIME_ADDRESS_TYPE_BCC", "5"),
    ("(gint) GMIME_ADDRESS_TYPE_CC", "4"),
    ("(gint) GMIME_ADDRESS_TYPE_FROM", "1"),
    ("(gint) GMIME_ADDRESS_TYPE_REPLY_TO", "2"),
    ("(gint) GMIME_ADDRESS_TYPE_SENDER", "0"),
    ("(gint) GMIME_ADDRESS_TYPE_TO", "3"),
    ("(gint) GMIME_AUTOCRYPT_PREFER_ENCRYPT_MUTUAL", "1"),
    ("(gint) GMIME_AUTOCRYPT_PREFER_ENCRYPT_NONE", "0"),
    ("GMIME_BINARY_AGE", "200"),
    ("(gint) GMIME_CIPHER_ALGO_3DES", "2"),
    ("(gint) GMIME_CIPHER_ALGO_AES", "7"),
    ("(gint) GMIME_CIPHER_ALGO_AES192", "8"),
    ("(gint) GMIME_CIPHER_ALGO_AES256", "9"),
    ("(gint) GMIME_CIPHER_ALGO_BLOWFISH", "4"),
    ("(gint) GMIME_CIPHER_ALGO_CAMELLIA128", "11"),
    ("(gint) GMIME_CIPHER_ALGO_CAMELLIA192", "12"),
    ("(gint) GMIME_CIPHER_ALGO_CAMELLIA256", "13"),
    ("(gint) GMIME_CIPHER_ALGO_CAST5", "3"),
    ("(gint) GMIME_CIPHER_ALGO_DEFAULT", "0"),
    ("(gint) GMIME_CIPHER_ALGO_IDEA", "1"),
    ("(gint) GMIME_CIPHER_ALGO_TWOFISH", "10"),
    ("(gint) GMIME_CONTENT_ENCODING_7BIT", "1"),
    ("(gint) GMIME_CONTENT_ENCODING_8BIT", "2"),
    ("(gint) GMIME_CONTENT_ENCODING_BASE64", "4"),
    ("(gint) GMIME_CONTENT_ENCODING_BINARY", "3"),
    ("(gint) GMIME_CONTENT_ENCODING_DEFAULT", "0"),
    ("(gint) GMIME_CONTENT_ENCODING_QUOTEDPRINTABLE", "5"),
    ("(gint) GMIME_CONTENT_ENCODING_UUENCODE", "6"),
    ("(gint) GMIME_CRIT_CONFLICTING_CONTENT_HDR", "9"),
    ("(gint) GMIME_CRIT_CONFLICTING_PARAMETER", "10"),
    ("(gint) GMIME_CRIT_MULTIPART_WITHOUT_BOUNDARY", "11"),
    ("(guint) GMIME_DECRYPT_EXPORT_SESSION_KEY", "1"),
    ("(guint) GMIME_DECRYPT_NONE", "0"),
    ("(gint) GMIME_DIGEST_ALGO_CRC32", "302"),
    ("(gint) GMIME_DIGEST_ALGO_CRC32_RFC1510", "303"),
    ("(gint) GMIME_DIGEST_ALGO_CRC32_RFC2440", "304"),
    ("(gint) GMIME_DIGEST_ALGO_DEFAULT", "0"),
    ("(gint) GMIME_DIGEST_ALGO_HAVAL5160", "7"),
    ("(gint) GMIME_DIGEST_ALGO_MD2", "5"),
    ("(gint) GMIME_DIGEST_ALGO_MD4", "301"),
    ("(gint) GMIME_DIGEST_ALGO_MD5", "1"),
    ("(gint) GMIME_DIGEST_ALGO_RIPEMD160", "3"),
    ("(gint) GMIME_DIGEST_ALGO_SHA1", "2"),
    ("(gint) GMIME_DIGEST_ALGO_SHA224", "11"),
    ("(gint) GMIME_DIGEST_ALGO_SHA256", "8"),
    ("(gint) GMIME_DIGEST_ALGO_SHA384", "9"),
    ("(gint) GMIME_DIGEST_ALGO_SHA512", "10"),
    ("(gint) GMIME_DIGEST_ALGO_TIGER192", "6"),
    ("GMIME_DISPOSITION_ATTACHMENT", "attachment"),
    ("GMIME_DISPOSITION_INLINE", "inline"),
    ("(gint) GMIME_ENCODING_CONSTRAINT_7BIT", "0"),
    ("(gint) GMIME_ENCODING_CONSTRAINT_8BIT", "1"),
    ("(gint) GMIME_ENCODING_CONSTRAINT_BINARY", "2"),
    ("(gint) GMIME_ENCRYPT_ALWAYS_TRUST", "1"),
    ("(gint) GMIME_ENCRYPT_NONE", "0"),
    ("(gint) GMIME_ENCRYPT_NO_COMPRESS", "16"),
    ("(gint) GMIME_ENCRYPT_SYMMETRIC", "32"),
    ("(gint) GMIME_ENCRYPT_THROW_KEYIDS", "64"),
    ("(guint) GMIME_FILTER_BEST_CHARSET", "1"),
    ("(guint) GMIME_FILTER_BEST_ENCODING", "2"),
    ("GMIME_FILTER_ENRICHED_IS_RICHTEXT", "1"),
    ("(gint) GMIME_FILTER_FROM_MODE_ARMOR", "1"),
    ("(gint) GMIME_FILTER_FROM_MODE_DEFAULT", "0"),
    ("(gint) GMIME_FILTER_FROM_MODE_ESCAPE", "0"),
    ("(gint) GMIME_FILTER_GZIP_MODE_UNZIP", "1"),
    ("(gint) GMIME_FILTER_GZIP_MODE_ZIP", "0"),
    ("GMIME_FILTER_HTML_BLOCKQUOTE_CITATION", "256"),
    ("GMIME_FILTER_HTML_CITE", "128"),
    ("GMIME_FILTER_HTML_CONVERT_ADDRESSES", "32"),
    ("GMIME_FILTER_HTML_CONVERT_NL", "2"),
    ("GMIME_FILTER_HTML_CONVERT_SPACES", "4"),
    ("GMIME_FILTER_HTML_CONVERT_URLS", "8"),
    ("GMIME_FILTER_HTML_ESCAPE_8BIT", "64"),
    ("GMIME_FILTER_HTML_MARK_CITATION", "16"),
    ("GMIME_FILTER_HTML_PRE", "1"),
    ("(gint) GMIME_FORMAT_MBOX", "1"),
    ("(gint) GMIME_FORMAT_MESSAGE", "0"),
    ("(gint) GMIME_FORMAT_MMDF", "2"),
    ("GMIME_INTERFACE_AGE", "0"),
    ("GMIME_MAJOR_VERSION", "3"),
    ("GMIME_MICRO_VERSION", "0"),
    ("GMIME_MINOR_VERSION", "2"),
    ("(gint) GMIME_NEWLINE_FORMAT_DOS", "1"),
    ("(gint) GMIME_NEWLINE_FORMAT_UNIX", "0"),
    ("(guint) GMIME_OPENPGP_BEGIN_PGP_MESSAGE", "1"),
    ("(guint) GMIME_OPENPGP_BEGIN_PGP_PRIVATE_KEY_BLOCK", "128"),
    ("(guint) GMIME_OPENPGP_BEGIN_PGP_PUBLIC_KEY_BLOCK", "32"),
    ("(guint) GMIME_OPENPGP_BEGIN_PGP_SIGNATURE", "12"),
    ("(guint) GMIME_OPENPGP_BEGIN_PGP_SIGNED_MESSAGE", "4"),
    ("(gint) GMIME_OPENPGP_DATA_ENCRYPTED", "1"),
    ("(gint) GMIME_OPENPGP_DATA_NONE", "0"),
    ("(gint) GMIME_OPENPGP_DATA_PRIVATE_KEY", "4"),
    ("(gint) GMIME_OPENPGP_DATA_PUBLIC_KEY", "3"),
    ("(gint) GMIME_OPENPGP_DATA_SIGNED", "2"),
    ("(guint) GMIME_OPENPGP_END_PGP_MESSAGE", "3"),
    ("(guint) GMIME_OPENPGP_END_PGP_PRIVATE_KEY_BLOCK", "384"),
    ("(guint) GMIME_OPENPGP_END_PGP_PUBLIC_KEY_BLOCK", "96"),
    ("(guint) GMIME_OPENPGP_END_PGP_SIGNATURE", "28"),
    ("(guint) GMIME_OPENPGP_NONE", "0"),
    ("(gint) GMIME_PARAM_ENCODING_METHOD_DEFAULT", "0"),
    ("(gint) GMIME_PARAM_ENCODING_METHOD_RFC2047", "2"),
    ("(gint) GMIME_PARAM_ENCODING_METHOD_RFC2231", "1"),
    ("(gint) GMIME_PUBKEY_ALGO_DEFAULT", "0"),
    ("(gint) GMIME_PUBKEY_ALGO_DSA", "17"),
    ("(gint) GMIME_PUBKEY_ALGO_ECC", "18"),
    ("(gint) GMIME_PUBKEY_ALGO_ECDH", "302"),
    ("(gint) GMIME_PUBKEY_ALGO_ECDSA", "301"),
    ("(gint) GMIME_PUBKEY_ALGO_EDDSA", "303"),
    ("(gint) GMIME_PUBKEY_ALGO_ELG", "20"),
    ("(gint) GMIME_PUBKEY_ALGO_ELG_E", "16"),
    ("(gint) GMIME_PUBKEY_ALGO_RSA", "1"),
    ("(gint) GMIME_PUBKEY_ALGO_RSA_E", "2"),
    ("(gint) GMIME_PUBKEY_ALGO_RSA_S", "3"),
    ("(gint) GMIME_RFC_COMPLIANCE_LOOSE", "0"),
    ("(gint) GMIME_RFC_COMPLIANCE_STRICT", "1"),
    ("(gint) GMIME_SECURE_MIME_TYPE_CERTS_ONLY", "3"),
    ("(gint) GMIME_SECURE_MIME_TYPE_COMPRESSED_DATA", "0"),
    ("(gint) GMIME_SECURE_MIME_TYPE_ENVELOPED_DATA", "1"),
    ("(gint) GMIME_SECURE_MIME_TYPE_SIGNED_DATA", "2"),
    ("(gint) GMIME_SECURE_MIME_TYPE_UNKNOWN", "4"),
    ("(gint) GMIME_SIGNATURE_STATUS_BAD_POLICY", "1024"),
    ("(gint) GMIME_SIGNATURE_STATUS_CRL_MISSING", "256"),
    ("(gint) GMIME_SIGNATURE_STATUS_CRL_TOO_OLD", "512"),
    ("GMIME_SIGNATURE_STATUS_ERROR_MASK", "-1"),
    ("(gint) GMIME_SIGNATURE_STATUS_GREEN", "2"),
    ("(gint) GMIME_SIGNATURE_STATUS_KEY_EXPIRED", "32"),
    ("(gint) GMIME_SIGNATURE_STATUS_KEY_MISSING", "128"),
    ("(gint) GMIME_SIGNATURE_STATUS_KEY_REVOKED", "16"),
    ("(gint) GMIME_SIGNATURE_STATUS_RED", "4"),
    ("(gint) GMIME_SIGNATURE_STATUS_SIG_EXPIRED", "64"),
    ("(gint) GMIME_SIGNATURE_STATUS_SYS_ERROR", "2048"),
    ("(gint) GMIME_SIGNATURE_STATUS_TOFU_CONFLICT", "4096"),
    ("(gint) GMIME_SIGNATURE_STATUS_VALID", "1"),
    ("(gint) GMIME_STREAM_BUFFER_BLOCK_READ", "0"),
    ("(gint) GMIME_STREAM_BUFFER_BLOCK_WRITE", "1"),
    ("(gint) GMIME_STREAM_SEEK_CUR", "1"),
    ("(gint) GMIME_STREAM_SEEK_END", "2"),
    ("(gint) GMIME_STREAM_SEEK_SET", "0"),
    ("(gint) GMIME_TRUST_FULL", "4"),
    ("(gint) GMIME_TRUST_MARGINAL", "3"),
    ("(gint) GMIME_TRUST_NEVER", "2"),
    ("(gint) GMIME_TRUST_ULTIMATE", "5"),
    ("(gint) GMIME_TRUST_UNDEFINED", "1"),
    ("(gint) GMIME_TRUST_UNKNOWN", "0"),
    ("GMIME_UUDECODE_STATE_BEGIN", "65536"),
    ("GMIME_UUDECODE_STATE_END", "131072"),
    ("GMIME_UUDECODE_STATE_INIT", "0"),
    ("GMIME_UUDECODE_STATE_MASK", "0"),
    ("(gint) GMIME_VALIDITY_FULL", "4"),
    ("(gint) GMIME_VALIDITY_MARGINAL", "3"),
    ("(gint) GMIME_VALIDITY_NEVER", "2"),
    ("(gint) GMIME_VALIDITY_ULTIMATE", "5"),
    ("(gint) GMIME_VALIDITY_UNDEFINED", "1"),
    ("(gint) GMIME_VALIDITY_UNKNOWN", "0"),
    ("(gint) GMIME_VERIFY_NONE", "0"),
    ("(gint) GMIME_WARN_DUPLICATED_CONTENT_HDR", "1"),
    ("(gint) GMIME_WARN_DUPLICATED_PARAMETER", "2"),
    ("(gint) GMIME_WARN_INVALID_CONTENT_TYPE", "4"),
    ("(gint) GMIME_WARN_INVALID_HEADER", "5"),
    ("(gint) GMIME_WARN_MALFORMED_MESSAGE", "8"),
    ("(gint) GMIME_WARN_MALFORMED_MULTIPART", "6"),
    ("(gint) GMIME_WARN_TRUNCATED_MESSAGE", "7"),
    ("(gint) GMIME_WARN_UNENCODED_8BIT_HEADER", "3"),
    ("GMIME_YDECODE_STATE_BEGIN", "4096"),
    ("GMIME_YDECODE_STATE_DECODE", "16384"),
    ("GMIME_YDECODE_STATE_END", "32768"),
    ("GMIME_YDECODE_STATE_EOLN", "256"),
    ("GMIME_YDECODE_STATE_ESCAPE", "512"),
    ("GMIME_YDECODE_STATE_INIT", "0"),
    ("GMIME_YDECODE_STATE_PART", "8192"),
    ("GMIME_YENCODE_CRC_INIT", "-1"),
    ("GMIME_YENCODE_STATE_INIT", "0"),
];


