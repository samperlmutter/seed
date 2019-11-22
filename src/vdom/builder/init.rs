use crate::{
    dom_types::View,
    orders::OrdersContainer,
    routing::Url,
    vdom::builder::{
        after_mount::{AfterMount, Into as IntoAfterMount, UrlHandling},
        before_mount::MountType,
    },
};

/// Used as a flexible wrapper for the init function.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[deprecated(
    since = "0.5.0",
    note = "Part of old Init API. Use a combination of `BeforeMount` and `AfterMount` instead."
)]
pub struct Init<Mdl> {
    /// Initial model to be used when the app begins.
    #[deprecated(
        since = "0.5.0",
        note = "Part of old Init API. Use `AfterMount` instead."
    )]
    pub model: Mdl,
    /// How to handle initial url routing. Defaults to [`UrlHandling::PassToRoutes`] in the
    /// constructors.
    #[deprecated(
        since = "0.5.0",
        note = "Part of old Init API. Use `AfterMount` instead."
    )]
    pub url_handling: UrlHandling,
    /// How to handle elements already present in the mount. Defaults to [`MountType::Append`]
    /// in the constructors.
    #[deprecated(
        since = "0.5.0",
        note = "Part of old Init API. Use `BeforeMount` instead."
    )]
    pub mount_type: MountType,
}

impl<Mdl> Init<Mdl> {
    #[deprecated(
        since = "0.5.0",
        note = "Part of old Init API. Use `AfterMount` instead."
    )]
    pub const fn new(model: Mdl) -> Self {
        Self {
            model,
            url_handling: UrlHandling::PassToRoutes,
            mount_type: MountType::Append,
        }
    }

    #[deprecated(
        since = "0.5.0",
        note = "Part of old Init API. Use `AfterMount` instead."
    )]
    pub const fn new_with_url_handling(model: Mdl, url_handling: UrlHandling) -> Self {
        Self {
            model,
            url_handling,
            mount_type: MountType::Append,
        }
    }
}

#[deprecated(
    since = "0.5.0",
    note = "Part of old Init API. Use `AfterMount` instead."
)]
pub type Fn<Ms, Mdl, ElC, GMs> =
    Box<dyn FnOnce(Url, &mut OrdersContainer<Ms, Mdl, ElC, GMs>) -> Init<Mdl>>;

#[deprecated(
    since = "0.5.0",
    note = "Part of old Init API. Use `IntoAfterMount` and `IntoBeforeMount` instead."
)]
pub trait Into<Ms: 'static, Mdl, ElC: View<Ms>, GMs> {
    fn into_init(self, init_url: Url, ord: &mut OrdersContainer<Ms, Mdl, ElC, GMs>) -> Init<Mdl>;
}

impl<Ms: 'static, Mdl, ElC: View<Ms>, GMs, F> Into<Ms, Mdl, ElC, GMs> for F
where
    F: FnOnce(Url, &mut OrdersContainer<Ms, Mdl, ElC, GMs>) -> Init<Mdl>,
{
    fn into_init(self, init_url: Url, ord: &mut OrdersContainer<Ms, Mdl, ElC, GMs>) -> Init<Mdl> {
        self(init_url, ord)
    }
}

impl<Ms: 'static, Mdl, ElC: View<Ms>, GMs> IntoAfterMount<Ms, Mdl, ElC, GMs>
    for (Init<Mdl>, OrdersContainer<Ms, Mdl, ElC, GMs>)
{
    fn into_after_mount(
        self: Box<Self>,
        _: Url,
        ord: &mut OrdersContainer<Ms, Mdl, ElC, GMs>,
    ) -> AfterMount<Mdl> {
        let (init, old_ord) = *self;
        ord.effects = old_ord.effects;
        ord.should_render = old_ord.should_render;
        AfterMount {
            model: init.model,
            url_handling: init.url_handling,
        }
    }
}
