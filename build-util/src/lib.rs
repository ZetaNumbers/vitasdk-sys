#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "build-dep")]
#[cfg_attr(docsrs, doc(cfg(feature = "build-dep")))]
pub mod link_visitor;
pub mod vita_headers_db;
