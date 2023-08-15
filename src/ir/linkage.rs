
#[derive(Eq, Debug, Clone)]
pub enum Linkage {
    ExternalLinkage,
    InternalLinkage,
    PrivateLinkage,
    ExternalWeakLinkage,
    CommonLinkage,
    AppendingLinkage,
    LinkonceLinkage,
    WeakLinkage,
}

impl PartialEq for Linkage {
    fn eq(&self, other: &Linkage) -> bool {
        match (self, other) {
            (&Linkage::ExternalLinkage, &Linkage::ExternalLinkage) => true,
            (&Linkage::InternalLinkage, &Linkage::InternalLinkage) => true,
            (&Linkage::PrivateLinkage, &Linkage::PrivateLinkage) => true,
            (&Linkage::ExternalWeakLinkage, &Linkage::ExternalWeakLinkage) => true,
            (&Linkage::CommonLinkage, &Linkage::CommonLinkage) => true,
            (&Linkage::AppendingLinkage, &Linkage::AppendingLinkage) => true,
            (&Linkage::LinkonceLinkage, &Linkage::LinkonceLinkage) => true,
            (&Linkage::WeakLinkage, &Linkage::WeakLinkage) => true,
            _ => false,
        }
    }
}
