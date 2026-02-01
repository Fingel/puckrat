use alpm::{Alpm, PackageReason, PackageValidation, SigLevel};

pub struct PackageInfo {
    pub version: String,
    pub description: String,
    pub arch: Option<String>,
    pub url: Option<String>,
    pub licenses: Vec<String>,
    pub groups: Vec<String>,
    pub provides: Vec<String>,
    pub conflics_with: Vec<String>,
    pub replaces: Vec<String>,
    pub installed_size: i64,
    pub packager: Option<String>,
    pub build_date: i64,
    pub install_date: Option<i64>,
    pub install_reason: PackageReason,
    pub install_script: bool,
    pub validated_by: PackageValidation,
}

pub struct Dependencies {
    pub depends_on: Vec<String>,
    pub optional_deps: Vec<String>,
    pub required_by: Vec<String>,
    pub optional_for: Vec<String>,
}

#[derive(Debug)]
pub struct AlpmService {
    handle: Alpm,
}

impl AlpmService {
    pub fn new() -> Result<Self, alpm::Error> {
        let handle = Alpm::new("/", "/var/lib/pacman")?;
        handle
            .register_syncdb("core", SigLevel::USE_DEFAULT)
            .unwrap();
        handle
            .register_syncdb("extra", SigLevel::USE_DEFAULT)
            .unwrap();
        handle
            .register_syncdb("community", SigLevel::USE_DEFAULT)
            .unwrap();

        Ok(Self { handle })
    }

    fn find_package(&self, package_name: &str) -> Result<&alpm::Package, alpm::Error> {
        for db in self.handle.syncdbs() {
            if let Ok(pkg) = db.pkg(package_name) {
                return Ok(pkg);
            }
        }

        Err(alpm::Error::PkgNotFound)
    }

    pub fn package_info(&self, package_name: &str) -> Result<PackageInfo, alpm::Error> {
        let pkg = self.find_package(package_name)?;
        Ok(PackageInfo {
            version: pkg.version().to_string(),
            description: pkg.desc().unwrap_or("None").to_string(),
            arch: pkg.arch().map(|arch| arch.to_string()),
            url: pkg.url().map(|url| url.to_string()),
            licenses: pkg
                .licenses()
                .iter()
                .map(|license| license.to_string())
                .collect(),
            groups: pkg.groups().iter().map(|group| group.to_string()).collect(),
            provides: pkg
                .provides()
                .iter()
                .map(|provide| provide.to_string())
                .collect(),
            conflics_with: pkg
                .conflicts()
                .iter()
                .map(|conflicts_with| conflicts_with.to_string())
                .collect(),
            replaces: pkg
                .replaces()
                .iter()
                .map(|replaces| replaces.to_string())
                .collect(),
            installed_size: pkg.size(),
            packager: pkg.packager().map(|packager| packager.to_string()),
            build_date: pkg.build_date(),
            install_date: pkg.install_date(),
            install_reason: pkg.reason(),
            install_script: pkg.has_scriptlet(),
            validated_by: pkg.validation(),
        })
    }

    pub fn dependencies(&self, package_name: &str) -> Result<Dependencies, alpm::Error> {
        let pkg = self.find_package(package_name)?;
        let depends_on = pkg.depends().iter().map(|dep| dep.to_string()).collect();
        let optional_deps = pkg.optdepends().iter().map(|dep| dep.to_string()).collect();
        let required_by = pkg
            .required_by()
            .iter()
            .map(|dep| dep.to_string())
            .collect();
        let optional_for = pkg
            .optional_for()
            .iter()
            .map(|dep| dep.to_string())
            .collect();
        Ok(Dependencies {
            depends_on,
            optional_deps,
            required_by,
            optional_for,
        })
    }
}
