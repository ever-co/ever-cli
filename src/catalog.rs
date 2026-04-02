#[derive(Debug, Clone, Copy)]
pub struct CatalogEntry {
    pub product: &'static str,
    pub npm_package: &'static str,
    pub crate_name: &'static str,
    pub repository: &'static str,
}

pub const PRODUCT_CATALOG: &[CatalogEntry] = &[
    CatalogEntry {
        product: "works",
        npm_package: "ever-works-cli",
        crate_name: "ever-works-cli",
        repository: "ever-co/ever-works-cli",
    },
    CatalogEntry {
        product: "gauzy",
        npm_package: "ever-gauzy-cli",
        crate_name: "ever-gauzy-cli",
        repository: "ever-co/ever-gauzy-cli",
    },
    CatalogEntry {
        product: "os",
        npm_package: "ever-os-cli",
        crate_name: "ever-os-cli",
        repository: "ever-co/ever-os-cli",
    },
    CatalogEntry {
        product: "teams",
        npm_package: "ever-teams-cli",
        crate_name: "ever-teams-cli",
        repository: "ever-co/ever-teams-cli",
    },
    CatalogEntry {
        product: "cloc",
        npm_package: "ever-cloc-cli",
        crate_name: "ever-cloc-cli",
        repository: "ever-co/ever-cloc-cli",
    },
];

pub fn find(product: &str) -> Option<&'static CatalogEntry> {
    PRODUCT_CATALOG.iter().find(|entry| entry.product == product)
}
