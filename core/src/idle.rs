//! Leerlaufzeit von Maus/Tastatur aus `ioreg -c IOHIDSystem -d 4`.

pub const IOREG_ARGS: &[&str] = &["-c", "IOHIDSystem", "-d", "4"];

/// Liefert die Leerlaufzeit in Sekunden (`HIDIdleTime` ist in Nanosekunden).
pub fn parse_ioreg(output: &str) -> Option<u64> {
    const KEY: &str = "\"HIDIdleTime\" = ";
    output.lines().find_map(|l| {
        let rest = &l[l.find(KEY)? + KEY.len()..];
        rest.trim().parse::<u64>().ok().map(|ns| ns / 1_000_000_000)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_idle_seconds() {
        let out = r#"+-o IOHIDSystem  <class IOHIDSystem, id 0x100000465>
    {
      "HIDIdleTime" = 125000000000
      "HIDParameters" = {"HIDClickTime"=500000000}
    }"#;
        assert_eq!(parse_ioreg(out), Some(125));
        assert_eq!(parse_ioreg("  | |   \"HIDIdleTime\" = 3000000000"), Some(3));
        assert_eq!(parse_ioreg("nichts"), None);
    }
}
