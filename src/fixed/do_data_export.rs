use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::fixed::Statics;

fn export<K, V>(filename: &str, value: &HashMap<K, V>) -> anyhow::Result<()>
where
    K: serde::Serialize + std::cmp::Ord,
    V: serde::Serialize,
{
    use std::collections::BTreeMap;
    let ordered = value.iter().collect::<BTreeMap<_, _>>();

    let json_str = serde_json::to_string_pretty(&ordered)?;
    write_different(&format!("static/{}.json", filename), &json_str)?;

    let yaml_str = serde_yaml::to_string(&ordered)?;
    write_different(&format!("static/{}.yaml", filename), &yaml_str)?;

    Ok(())
}

fn write_different<P: AsRef<Path>>(path: P, contents: &str) -> std::io::Result<()> {
    let path = path.as_ref();
    let current = fs::read_to_string(path).unwrap_or_default();
    if current != contents {
        fs::write(path, contents)?;
    }
    Ok(())
}

fn graphviz(format: &str, filename: &str) -> anyhow::Result<()> {
    let output = std::process::Command::new("dot")
        .arg(format!("-T{}", format))
        .arg(format!("{}.graphviz", filename))
        .current_dir("static")
        .output()?;

    if output.status.success() {
        fs::write(format!("static/{}.{}", filename, format), output.stdout)?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("failed?"))
    }
}

#[test]
fn check_facility() -> anyhow::Result<()> {
    let all = Statics::default().facilities;
    assert!(!all.data.is_empty(), "is empty");
    export("facility", &all.data)
}

#[test]
fn check_lifeless() -> anyhow::Result<()> {
    let all = Statics::default().lifeless;
    assert!(!all.data.is_empty(), "is empty");

    for (key, value) in &all.data {
        assert!(value.collateral.is_alive(), "collateral {:?}", key);
    }

    export("lifeless", &all.data)
}

#[test]
fn check_module_passive() -> anyhow::Result<()> {
    let all = Statics::default().modules_passive;
    assert!(!all.data.is_empty(), "is empty");

    for (key, value) in &all.data {
        assert!(
            value.required_cpu.saturating_add(value.required_powergrid) > 0,
            "require {:?}",
            key
        );
        // TODO: ensure some attribute does something
    }

    export("module-passive", &all.data)
}

#[test]
fn check_module_untargeted() -> anyhow::Result<()> {
    let all = Statics::default().modules_untargeted;
    assert!(!all.data.is_empty(), "is empty");

    for (key, value) in &all.data {
        assert!(
            value.required_cpu.saturating_add(value.required_powergrid) > 0,
            "require {:?}",
            key
        );
        assert!(!value.effects.is_empty(), "effect {:?}", key);
    }

    export("module-untargeted", &all.data)
}

#[test]
fn check_module_targeted() -> anyhow::Result<()> {
    let all = Statics::default().modules_targeted;
    assert!(!all.data.is_empty(), "is empty");

    for (key, value) in &all.data {
        assert!(
            value.required_cpu.saturating_add(value.required_powergrid) > 0,
            "requires {:?}",
            key
        );
        let total_effects = value.effects_origin.len() + value.effects_target.len();
        assert_ne!(total_effects, 0, "effects {:?}", key);
    }

    export("module-targeted", &all.data)
}

#[test]
fn check_ship_layout() -> anyhow::Result<()> {
    let all = Statics::default().ship_layouts;
    assert!(!all.data.is_empty(), "is empty");

    for (key, value) in &all.data {
        assert!(value.collateral.capacitor > 0, "capacitor {:?}", key);
        assert!(value.collateral.is_alive(), "alive {:?}", key);
    }

    export("ship-layout", &all.data)
}

#[test]
fn check_solarsystem() -> anyhow::Result<()> {
    let all = Statics::default().solarsystems;
    assert!(!all.data.is_empty(), "is empty");

    for (key, value) in &all.data {
        assert!(value.security <= 100, "security {:?}", key);

        for planet in value.stargates.values() {
            assert!(planet <= &value.planets, "stargate planet {:?}", key);
        }

        for planet in &value.stations {
            assert!(planet <= &value.planets, "station planet {:?}", key);
        }
    }

    export("solarsystem", &all.data)
}

#[test]
fn graphviz_solarsystems() -> anyhow::Result<()> {
    let mut text = String::new();
    text += "digraph {\n";

    let all = Statics::default().solarsystems;

    let mut ordered = all.data.iter().collect::<Vec<_>>();
    ordered.sort_by(|a, b| b.1.security.cmp(&a.1.security));

    for (key, system) in ordered {
        text += "\t";
        text += &format!(r#"{:?}[label="{} ({})"];"#, key, key, system.security);
        text += "\n";

        for target in system.stargates.keys() {
            text += &format!("\t{:?} -> {:?};\n", key, target);
        }
    }

    text += "}\n";
    fs::write("static/solarsystems.graphviz", &text)?;
    drop(graphviz("svg", "solarsystems"));
    drop(graphviz("png", "solarsystems"));
    Ok(())
}
