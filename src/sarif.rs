//! SARIF 2.1.0 output support for CI/CD
use serde_json::json;

pub struct Vuln {
      pub rule: String,
      pub msg: String,
      pub file: String,
      pub line: u32,
}

pub fn to_sarif(vulns: &[Vuln]) -> String {
      let run = json!({
                "tool": {"driver": {"name": "AuditPulse"}},
                "results": vulns.iter().map(|v| json!({
                              "ruleId": v.rule,
                              "level": "error",
                              "message": {"text": v.msg},
                              "locations": [{"physicalLocation": {
                                                "artifactLocation": {"uri": v.file},
                                                "region": {"startLine": v.line}
                              }}]
                })).collect::<Vec<_>>()
      });
      serde_json::to_string_pretty(&json!({"version":"2.1.0","runs":[run]})).unwrap()
}
