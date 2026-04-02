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
    CatalogEntry {
        product: "rec",
        npm_package: "ever-rec-cli",
        crate_name: "ever-rec-cli",
        repository: "ever-co/ever-rec-cli",
    },
    CatalogEntry {
        product: "hust",
        npm_package: "ever-hust-cli",
        crate_name: "ever-hust-cli",
        repository: "ever-co/ever-hust-cli",
    },
    CatalogEntry {
        product: "jobs",
        npm_package: "ever-jobs-cli",
        crate_name: "ever-jobs-cli",
        repository: "ever-co/ever-jobs-cli",
    },
    CatalogEntry {
        product: "dev",
        npm_package: "ever-dev-cli",
        crate_name: "ever-dev-cli",
        repository: "ever-co/ever-dev-cli",
    },
    CatalogEntry {
        product: "demand",
        npm_package: "ever-demand-cli",
        crate_name: "ever-demand-cli",
        repository: "ever-co/ever-demand-cli",
    },
    CatalogEntry {
        product: "traduora",
        npm_package: "ever-traduora-cli",
        crate_name: "ever-traduora-cli",
        repository: "ever-co/ever-traduora-cli",
    },
    CatalogEntry {
        product: "tech",
        npm_package: "ever-tech-cli",
        crate_name: "ever-tech-cli",
        repository: "ever-co/ever-tech-cli",
    },
    CatalogEntry {
        product: "saas",
        npm_package: "ever-saas-cli",
        crate_name: "ever-saas-cli",
        repository: "ever-co/ever-saas-cli",
    },
    CatalogEntry {
        product: "docs",
        npm_package: "ever-docs-cli",
        crate_name: "ever-docs-cli",
        repository: "ever-co/ever-docs-cli",
    },
    CatalogEntry {
        product: "digital",
        npm_package: "ever-digital-cli",
        crate_name: "ever-digital-cli",
        repository: "ever-co/ever-digital-cli",
    },
    CatalogEntry {
        product: "shop",
        npm_package: "ever-shop-cli",
        crate_name: "ever-shop-cli",
        repository: "ever-co/ever-shop-cli",
    },
    CatalogEntry {
        product: "iq",
        npm_package: "ever-iq-cli",
        crate_name: "ever-iq-cli",
        repository: "ever-co/ever-iq-cli",
    },
];

pub fn find(product: &str) -> Option<&'static CatalogEntry> {
    PRODUCT_CATALOG.iter().find(|entry| entry.product == product)
}

pub fn binary_name(product: &str) -> String {
    format!("ever-{product}")
}
