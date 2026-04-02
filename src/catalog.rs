#[derive(Debug, Clone, Copy)]
pub struct CatalogEntry {
    pub product: &'static str,
    pub npm_package: &'static str,
}

pub const PRODUCT_CATALOG: &[CatalogEntry] = &[
    CatalogEntry {
        product: "works",
        npm_package: "ever-works-cli",
    },
    CatalogEntry {
        product: "gauzy",
        npm_package: "ever-gauzy-cli",
    },
    CatalogEntry {
        product: "os",
        npm_package: "ever-os-cli",
    },
    CatalogEntry {
        product: "teams",
        npm_package: "ever-teams-cli",
    },
    CatalogEntry {
        product: "cloc",
        npm_package: "ever-cloc-cli",
    },
    CatalogEntry {
        product: "rec",
        npm_package: "ever-rec-cli",
    },
    CatalogEntry {
        product: "hust",
        npm_package: "ever-hust-cli",
    },
    CatalogEntry {
        product: "jobs",
        npm_package: "ever-jobs-cli",
    },
    CatalogEntry {
        product: "dev",
        npm_package: "ever-dev-cli",
    },
    CatalogEntry {
        product: "demand",
        npm_package: "ever-demand-cli",
    },
    CatalogEntry {
        product: "traduora",
        npm_package: "ever-traduora-cli",
    },
    CatalogEntry {
        product: "tech",
        npm_package: "ever-tech-cli",
    },
    CatalogEntry {
        product: "saas",
        npm_package: "ever-saas-cli",
    },
    CatalogEntry {
        product: "docs",
        npm_package: "ever-docs-cli",
    },
    CatalogEntry {
        product: "digital",
        npm_package: "ever-digital-cli",
    },
    CatalogEntry {
        product: "shop",
        npm_package: "ever-shop-cli",
    },
    CatalogEntry {
        product: "iq",
        npm_package: "ever-iq-cli",
    },
];

pub fn find(product: &str) -> Option<&'static CatalogEntry> {
    PRODUCT_CATALOG.iter().find(|entry| entry.product == product)
}

pub fn binary_name(product: &str) -> String {
    format!("ever-{product}")
}
