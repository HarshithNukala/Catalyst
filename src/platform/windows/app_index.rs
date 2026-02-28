use std::path::{PathBuf, Path};
use lnk::ShellLink;
use dirs::data_dir;
use lnk::encoding::WINDOWS_1252;

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub path: PathBuf,
    pub icon: Option<PathBuf>,
}

pub struct AppIndex {
    apps: Vec<AppEntry>,
}

fn parse_lnk(path: &Path) -> Option<AppEntry> {
    let link = ShellLink::open(path, WINDOWS_1252).ok()?;
    // we are using .as_ref() to convert the Option<ShellLink> to Option<&ShellLink>
    // this is because the link_info() method returns an Option<ShellLink>
    let target = link.link_info().as_ref()?.local_base_path()?.to_string();
    let target_path = PathBuf::from(&target);
    if target_path.extension()?.to_str()? != "exe" {
        return None;
    }
    let name = path.file_stem()?.to_string_lossy().to_string();
    let icon = link.string_data().icon_location().as_ref().map(|s| PathBuf::from(s));
    Some(AppEntry {
        name,
        path: target_path,
        icon,
    })
}

impl AppIndex {
    pub fn build() -> Self {
        let mut apps = Vec::new();
        let dirs = vec![
            // PathBuf since it is absolute
            PathBuf::from(r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs"),
            // dirs::data_dir() since it is relative. An alternative is to use std::env::home_dir() inside of PathBuf::from()
            dirs::data_dir().unwrap().join(r"Microsoft\Windows\Start Menu\Programs"),
            // paths for desktop applications
        ];
        for dir in dirs {
            Self::scan_dir(&dir, &mut apps);
        }

        let built_in_apps = vec![
            ("File Explorer", r"C:\Windows\explorer.exe"),
            ("Task Manager", r"C:\Windows\System32\Taskmgr.exe"),
            ("Command Prompt", r"C:\Windows\System32\cmd.exe"),
            ("Registry Editor", r"C:\Windows\regedit.exe"),
            ("Calculator", r"C:\Windows\System32\calc.exe"),
            ("Notepad", r"C:\Windows\System32\notepad.exe"),
            ("Paint", r"C:\Windows\System32\mspaint.exe"),
            ("Control Panel", r"C:\Windows\System32\control.exe"),
        ];
        for (name, path) in built_in_apps {
            let path_buf = PathBuf::from(path);
            if path_buf.exists() {
                apps.push(AppEntry { name: name.to_string(), path: path_buf, icon: None });
            }
        }

        apps.sort_by(|a, b| a.name.cmp(&b.name));
        apps.dedup_by(|a, b| a.name == b.name);
        Self { apps }
    }

    fn scan_dir(dir: &Path, apps: &mut Vec<AppEntry>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    Self::scan_dir(&path, apps);
                } else if path.extension().map_or(false, |e| e == "lnk") {
                    if let Some(app) = parse_lnk(&path) {
                        apps.push(app);
                    }
                }
            }
        }
    }

    pub fn search(&self, query: &str) -> Vec<AppEntry> {
        let query_lower = query.to_lowercase();
        self.apps.iter()
            .filter(|app| app.name.to_lowercase().contains(&query_lower))
            .cloned()
            .collect()
    }
}
