#[allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Windows {
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Data {
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Xml {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Dom {
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct DtdEntity(::windows::IInspectable);
                impl DtdEntity {
                    pub fn PublicId(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SystemId(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn NotationName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn NodeValue(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SetNodeValue<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeType(&self) -> ::windows::Result<NodeType> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <NodeType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<NodeType>(result__)
                        }
                    }
                    pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn ParentNode(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ChildNodes(&self) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn FirstChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LastChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn PreviousSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NextSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Attributes(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn HasChildNodes(&self) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn OwnerDocument(&self) -> ::windows::Result<XmlDocument> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlDocument as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocument>(result__)
                        }
                    }
                    pub fn InsertBefore<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ReplaceChild<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        childnode: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                childnode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn AppendChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        newchild: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn CloneNode(&self, deep: bool) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).23)(
                                ::std::mem::transmute_copy(this),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NamespaceUri(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).24)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn LocalName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).25)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Prefix(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).26)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Normalize(&self) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).27)(::std::mem::transmute_copy(
                                this,
                            ))
                            .ok()
                        }
                    }
                    pub fn SetPrefix<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).28)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for DtdEntity {
                    const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Data.Xml.Dom.DtdEntity;{6a0b5ffc-63b4-480f-9e6a-8a92816aade4})" ) ;
                }
                unsafe impl ::windows::Interface for DtdEntity {
                    type Vtable = IDtdEntity_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1779130364,
                        25524,
                        18447,
                        [158, 106, 138, 146, 129, 106, 173, 228],
                    );
                }
                impl ::windows::RuntimeName for DtdEntity {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdEntity";
                }
                impl ::std::convert::From<DtdEntity> for ::windows::IUnknown {
                    fn from(value: DtdEntity) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&DtdEntity> for ::windows::IUnknown {
                    fn from(value: &DtdEntity) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for DtdEntity {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &DtdEntity {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<DtdEntity> for ::windows::IInspectable {
                    fn from(value: DtdEntity) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&DtdEntity> for ::windows::IInspectable {
                    fn from(value: &DtdEntity) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for DtdEntity {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a DtdEntity {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<DtdEntity> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: DtdEntity) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&DtdEntity> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: &DtdEntity) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for DtdEntity {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for &DtdEntity {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::std::convert::TryInto::<IXmlNode>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<DtdEntity> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: DtdEntity) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&DtdEntity> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: &DtdEntity) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for DtdEntity {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for &DtdEntity {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<DtdEntity> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: DtdEntity) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&DtdEntity> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: &DtdEntity) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for DtdEntity {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for &DtdEntity {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                unsafe impl ::std::marker::Send for DtdEntity {}
                unsafe impl ::std::marker::Sync for DtdEntity {}
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct DtdNotation(::windows::IInspectable);
                impl DtdNotation {
                    pub fn PublicId(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SystemId(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn NodeValue(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SetNodeValue<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeType(&self) -> ::windows::Result<NodeType> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <NodeType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<NodeType>(result__)
                        }
                    }
                    pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn ParentNode(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ChildNodes(&self) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn FirstChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LastChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn PreviousSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NextSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Attributes(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn HasChildNodes(&self) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn OwnerDocument(&self) -> ::windows::Result<XmlDocument> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlDocument as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocument>(result__)
                        }
                    }
                    pub fn InsertBefore<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ReplaceChild<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        childnode: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                childnode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn AppendChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        newchild: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn CloneNode(&self, deep: bool) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).23)(
                                ::std::mem::transmute_copy(this),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NamespaceUri(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).24)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn LocalName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).25)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Prefix(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).26)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Normalize(&self) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).27)(::std::mem::transmute_copy(
                                this,
                            ))
                            .ok()
                        }
                    }
                    pub fn SetPrefix<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).28)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for DtdNotation {
                    const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Data.Xml.Dom.DtdNotation;{8cb4e04d-6d46-4edb-ab73-df83c51ad397})" ) ;
                }
                unsafe impl ::windows::Interface for DtdNotation {
                    type Vtable = IDtdNotation_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        2360664141,
                        27974,
                        20187,
                        [171, 115, 223, 131, 197, 26, 211, 151],
                    );
                }
                impl ::windows::RuntimeName for DtdNotation {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdNotation";
                }
                impl ::std::convert::From<DtdNotation> for ::windows::IUnknown {
                    fn from(value: DtdNotation) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&DtdNotation> for ::windows::IUnknown {
                    fn from(value: &DtdNotation) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for DtdNotation {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &DtdNotation {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<DtdNotation> for ::windows::IInspectable {
                    fn from(value: DtdNotation) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&DtdNotation> for ::windows::IInspectable {
                    fn from(value: &DtdNotation) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for DtdNotation {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a DtdNotation {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<DtdNotation> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: DtdNotation) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&DtdNotation> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: &DtdNotation) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for DtdNotation {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for &DtdNotation {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::std::convert::TryInto::<IXmlNode>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<DtdNotation> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: DtdNotation) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&DtdNotation> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: &DtdNotation) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for DtdNotation {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for &DtdNotation {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<DtdNotation> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: DtdNotation) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&DtdNotation> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: &DtdNotation) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for DtdNotation {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for &DtdNotation {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                unsafe impl ::std::marker::Send for DtdNotation {}
                unsafe impl ::std::marker::Sync for DtdNotation {}
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IDtdEntity(::windows::IInspectable);
                unsafe impl ::windows::Interface for IDtdEntity {
                    type Vtable = IDtdEntity_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1779130364,
                        25524,
                        18447,
                        [158, 106, 138, 146, 129, 106, 173, 228],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IDtdEntity_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IDtdNotation(::windows::IInspectable);
                unsafe impl ::windows::Interface for IDtdNotation {
                    type Vtable = IDtdNotation_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        2360664141,
                        27974,
                        20187,
                        [171, 115, 223, 131, 197, 26, 211, 151],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IDtdNotation_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlAttribute(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlAttribute {
                    type Vtable = IXmlAttribute_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        2887010980,
                        46321,
                        19894,
                        [178, 6, 138, 34, 195, 8, 219, 10],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlAttribute_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut bool,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlCDataSection(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlCDataSection {
                    type Vtable = IXmlCDataSection_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1292153967,
                        51389,
                        17844,
                        [136, 153, 4, 0, 215, 194, 198, 15],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlCDataSection_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct IXmlCharacterData(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlCharacterData {
                    type Vtable = IXmlCharacterData_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        321798827,
                        20022,
                        19958,
                        [177, 200, 12, 230, 47, 216, 139, 38],
                    );
                }
                impl IXmlCharacterData {
                    pub fn Data(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetData<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn Length(&self) -> ::windows::Result<u32> {
                        let this = self;
                        unsafe {
                            let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<u32>(result__)
                        }
                    }
                    pub fn SubstringData(
                        &self,
                        offset: u32,
                        count: u32,
                    ) -> ::windows::Result<::windows::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                count,
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn AppendData<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        data: Param0,
                    ) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                data.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn InsertData<'a, Param1: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        offset: u32,
                        data: Param1,
                    ) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                data.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                count,
                            )
                            .ok()
                        }
                    }
                    pub fn ReplaceData<'a, Param2: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        offset: u32,
                        count: u32,
                        data: Param2,
                    ) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                count,
                                data.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeValue(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SetNodeValue<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeType(&self) -> ::windows::Result<NodeType> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <NodeType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<NodeType>(result__)
                        }
                    }
                    pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn ParentNode(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ChildNodes(&self) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn FirstChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LastChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn PreviousSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NextSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Attributes(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn HasChildNodes(&self) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn OwnerDocument(&self) -> ::windows::Result<XmlDocument> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlDocument as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocument>(result__)
                        }
                    }
                    pub fn InsertBefore<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ReplaceChild<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        childnode: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                childnode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn AppendChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        newchild: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn CloneNode(&self, deep: bool) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).23)(
                                ::std::mem::transmute_copy(this),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NamespaceUri(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).24)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn LocalName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).25)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Prefix(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).26)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Normalize(&self) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).27)(::std::mem::transmute_copy(
                                this,
                            ))
                            .ok()
                        }
                    }
                    pub fn SetPrefix<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).28)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for IXmlCharacterData {
                    const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                        b"{132e42ab-4e36-4df6-b1c8-0ce62fd88b26}",
                    );
                }
                impl ::std::convert::From<IXmlCharacterData> for ::windows::IUnknown {
                    fn from(value: IXmlCharacterData) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&IXmlCharacterData> for ::windows::IUnknown {
                    fn from(value: &IXmlCharacterData) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IXmlCharacterData {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IXmlCharacterData {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<IXmlCharacterData> for ::windows::IInspectable {
                    fn from(value: IXmlCharacterData) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&IXmlCharacterData> for ::windows::IInspectable {
                    fn from(value: &IXmlCharacterData) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for IXmlCharacterData {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IXmlCharacterData {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<IXmlCharacterData> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: IXmlCharacterData) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&IXmlCharacterData> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: &IXmlCharacterData) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for IXmlCharacterData {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for &IXmlCharacterData {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::std::convert::TryInto::<IXmlNode>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<IXmlCharacterData> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: IXmlCharacterData) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&IXmlCharacterData> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: &IXmlCharacterData) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for IXmlCharacterData {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for &IXmlCharacterData {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<IXmlCharacterData> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: IXmlCharacterData) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&IXmlCharacterData> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: &IXmlCharacterData) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for IXmlCharacterData {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for &IXmlCharacterData {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlCharacterData_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut u32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        offset: u32,
                        count: u32,
                        result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        data: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        offset: u32,
                        data: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        offset: u32,
                        count: u32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        offset: u32,
                        count: u32,
                        data: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlComment(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlComment {
                    type Vtable = IXmlComment_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        3164894421,
                        46623,
                        17937,
                        [156, 172, 46, 146, 227, 71, 109, 71],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlComment_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlDocument(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlDocument {
                    type Vtable = IXmlDocument_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        4159939846,
                        7815,
                        17110,
                        [188, 251, 184, 200, 9, 250, 84, 148],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlDocument_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        tagname: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        data: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        data: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        target: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        data: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        name: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        name: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        tagname: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        data: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        namespaceuri: ::windows::RawPtr,
                        qualifiedname: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        namespaceuri: ::windows::RawPtr,
                        qualifiedname: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        elementid: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        node: ::windows::RawPtr,
                        deep: bool,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlDocumentFragment(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlDocumentFragment {
                    type Vtable = IXmlDocumentFragment_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        3807013526,
                        3105,
                        17573,
                        [139, 201, 158, 74, 38, 39, 8, 236],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlDocumentFragment_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlDocumentIO(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlDocumentIO {
                    type Vtable = IXmlDocumentIO_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1825630030,
                        61029,
                        17545,
                        [158, 191, 202, 67, 232, 123, 166, 55],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlDocumentIO_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        xml: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        xml: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        loadsettings: ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        file: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlDocumentIO2(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlDocumentIO2 {
                    type Vtable = IXmlDocumentIO2_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1560495713,
                        31704,
                        19157,
                        [158, 191, 129, 230, 52, 114, 99, 177],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlDocumentIO2_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        buffer: ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        buffer: ::windows::RawPtr,
                        loadsettings: ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlDocumentStatics(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlDocumentStatics {
                    type Vtable = IXmlDocumentStatics_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1430508116,
                        55127,
                        19321,
                        [149, 57, 35, 43, 24, 245, 11, 241],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlDocumentStatics_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        uri: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        uri: ::windows::RawPtr,
                        loadsettings: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        file: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        file: ::windows::RawPtr,
                        loadsettings: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlDocumentType(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlDocumentType {
                    type Vtable = IXmlDocumentType_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        4147389477,
                        38785,
                        18788,
                        [142, 148, 155, 28, 109, 252, 155, 199],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlDocumentType_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlDomImplementation(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlDomImplementation {
                    type Vtable = IXmlDomImplementation_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1843757362,
                        61725,
                        20411,
                        [140, 198, 88, 60, 186, 147, 17, 47],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlDomImplementation_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        feature: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        version: ::windows::RawPtr,
                        result__: *mut bool,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlElement(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlElement {
                    type Vtable = IXmlElement_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        771459615,
                        27408,
                        20216,
                        [159, 131, 239, 204, 232, 250, 236, 55],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlElement_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        attributename: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        attributename: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        attributevalue: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        attributename: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        attributename: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        newattribute: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        attributenode: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        tagname: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        namespaceuri: ::windows::RawPtr,
                        qualifiedname: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        value: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        namespaceuri: ::windows::RawPtr,
                        localname: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        namespaceuri: ::windows::RawPtr,
                        localname: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        newattribute: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        namespaceuri: ::windows::RawPtr,
                        localname: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlEntityReference(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlEntityReference {
                    type Vtable = IXmlEntityReference_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        774850492,
                        50128,
                        19663,
                        [187, 134, 10, 184, 195, 106, 97, 207],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlEntityReference_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlLoadSettings(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlLoadSettings {
                    type Vtable = IXmlLoadSettings_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1487538088,
                        65238,
                        18167,
                        [180, 197, 251, 27, 167, 33, 8, 214],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlLoadSettings_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut u32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: u32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut bool,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: bool,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut bool,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: bool,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut bool,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: bool,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut bool,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: bool,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlNamedNodeMap(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlNamedNodeMap {
                    type Vtable = IXmlNamedNodeMap_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        3014041264,
                        43696,
                        19330,
                        [166, 250, 177, 69, 63, 124, 2, 27],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlNamedNodeMap_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut u32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        index: u32,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        name: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        node: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        name: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        namespaceuri: ::windows::RawPtr,
                        name: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        namespaceuri: ::windows::RawPtr,
                        name: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        node: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct IXmlNode(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlNode {
                    type Vtable = IXmlNode_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        477371737,
                        8482,
                        18389,
                        [168, 86, 131, 243, 212, 33, 72, 117],
                    );
                }
                impl IXmlNode {
                    pub fn NodeValue(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SetNodeValue<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeType(&self) -> ::windows::Result<NodeType> {
                        let this = self;
                        unsafe {
                            let mut result__: <NodeType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<NodeType>(result__)
                        }
                    }
                    pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn ParentNode(&self) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ChildNodes(&self) -> ::windows::Result<XmlNodeList> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn FirstChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LastChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn PreviousSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NextSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Attributes(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn HasChildNodes(&self) -> ::windows::Result<bool> {
                        let this = self;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn OwnerDocument(&self) -> ::windows::Result<XmlDocument> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlDocument as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocument>(result__)
                        }
                    }
                    pub fn InsertBefore<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ReplaceChild<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        childnode: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                childnode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn AppendChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        newchild: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn CloneNode(&self, deep: bool) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).23)(
                                ::std::mem::transmute_copy(this),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NamespaceUri(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).24)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn LocalName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).25)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Prefix(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).26)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Normalize(&self) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).27)(::std::mem::transmute_copy(
                                this,
                            ))
                            .ok()
                        }
                    }
                    pub fn SetPrefix<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).28)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for IXmlNode {
                    const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                        b"{1c741d59-2122-47d5-a856-83f3d4214875}",
                    );
                }
                impl ::std::convert::From<IXmlNode> for ::windows::IUnknown {
                    fn from(value: IXmlNode) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&IXmlNode> for ::windows::IUnknown {
                    fn from(value: &IXmlNode) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IXmlNode {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IXmlNode {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<IXmlNode> for ::windows::IInspectable {
                    fn from(value: IXmlNode) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&IXmlNode> for ::windows::IInspectable {
                    fn from(value: &IXmlNode) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for IXmlNode {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IXmlNode {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<IXmlNode> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: IXmlNode) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&IXmlNode> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: &IXmlNode) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for IXmlNode {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for &IXmlNode {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<IXmlNode> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: IXmlNode) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&IXmlNode> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: &IXmlNode) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for IXmlNode {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for &IXmlNode {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlNode_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut NodeType,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut bool,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        newchild: ::windows::RawPtr,
                        referencechild: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        newchild: ::windows::RawPtr,
                        referencechild: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        childnode: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        newchild: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        deep: bool,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlNodeList(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlNodeList {
                    type Vtable = IXmlNodeList_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        2355146103,
                        33700,
                        20161,
                        [156, 84, 123, 164, 41, 225, 61, 166],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlNodeList_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut u32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        index: u32,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct IXmlNodeSelector(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlNodeSelector {
                    type Vtable = IXmlNodeSelector_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1675344523,
                        53467,
                        20449,
                        [183, 69, 249, 67, 58, 253, 194, 91],
                    );
                }
                impl IXmlNodeSelector {
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for IXmlNodeSelector {
                    const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                        b"{63dbba8b-d0db-4fe1-b745-f9433afdc25b}",
                    );
                }
                impl ::std::convert::From<IXmlNodeSelector> for ::windows::IUnknown {
                    fn from(value: IXmlNodeSelector) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&IXmlNodeSelector> for ::windows::IUnknown {
                    fn from(value: &IXmlNodeSelector) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IXmlNodeSelector {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IXmlNodeSelector {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<IXmlNodeSelector> for ::windows::IInspectable {
                    fn from(value: IXmlNodeSelector) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&IXmlNodeSelector> for ::windows::IInspectable {
                    fn from(value: &IXmlNodeSelector) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for IXmlNodeSelector {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IXmlNodeSelector {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlNodeSelector_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        xpath: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        xpath: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        xpath: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        namespaces: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        xpath: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                        namespaces: ::windows::RawPtr,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct IXmlNodeSerializer(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlNodeSerializer {
                    type Vtable = IXmlNodeSerializer_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1556460418,
                        59101,
                        18833,
                        [171, 239, 6, 216, 210, 231, 189, 12],
                    );
                }
                impl IXmlNodeSerializer {
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for IXmlNodeSerializer {
                    const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                        b"{5cc5b382-e6dd-4991-abef-06d8d2e7bd0c}",
                    );
                }
                impl ::std::convert::From<IXmlNodeSerializer> for ::windows::IUnknown {
                    fn from(value: IXmlNodeSerializer) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&IXmlNodeSerializer> for ::windows::IUnknown {
                    fn from(value: &IXmlNodeSerializer) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IXmlNodeSerializer {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IXmlNodeSerializer {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<IXmlNodeSerializer> for ::windows::IInspectable {
                    fn from(value: IXmlNodeSerializer) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&IXmlNodeSerializer> for ::windows::IInspectable {
                    fn from(value: &IXmlNodeSerializer) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for IXmlNodeSerializer {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IXmlNodeSerializer {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlNodeSerializer_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IXmlProcessingInstruction(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlProcessingInstruction {
                    type Vtable = IXmlProcessingInstruction_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        654834974,
                        7826,
                        20174,
                        [182, 244, 38, 240, 105, 7, 141, 220],
                    );
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlProcessingInstruction_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct IXmlText(::windows::IInspectable);
                unsafe impl ::windows::Interface for IXmlText {
                    type Vtable = IXmlText_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        4180780235,
                        12429,
                        18272,
                        [161, 213, 67, 182, 116, 80, 172, 126],
                    );
                }
                impl IXmlText {
                    pub fn SplitText(&self, offset: u32) -> ::windows::Result<IXmlText> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlText as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                &mut result__,
                            )
                            .from_abi::<IXmlText>(result__)
                        }
                    }
                    pub fn Data(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetData<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn Length(&self) -> ::windows::Result<u32> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<u32>(result__)
                        }
                    }
                    pub fn SubstringData(
                        &self,
                        offset: u32,
                        count: u32,
                    ) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                count,
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn AppendData<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        data: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                data.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn InsertData<'a, Param1: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        offset: u32,
                        data: Param1,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                data.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                count,
                            )
                            .ok()
                        }
                    }
                    pub fn ReplaceData<'a, Param2: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        offset: u32,
                        count: u32,
                        data: Param2,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                count,
                                data.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeValue(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SetNodeValue<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeType(&self) -> ::windows::Result<NodeType> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <NodeType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<NodeType>(result__)
                        }
                    }
                    pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn ParentNode(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ChildNodes(&self) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn FirstChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LastChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn PreviousSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NextSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Attributes(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn HasChildNodes(&self) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn OwnerDocument(&self) -> ::windows::Result<XmlDocument> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlDocument as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocument>(result__)
                        }
                    }
                    pub fn InsertBefore<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ReplaceChild<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        childnode: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                childnode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn AppendChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        newchild: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn CloneNode(&self, deep: bool) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).23)(
                                ::std::mem::transmute_copy(this),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NamespaceUri(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).24)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn LocalName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).25)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Prefix(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).26)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Normalize(&self) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).27)(::std::mem::transmute_copy(
                                this,
                            ))
                            .ok()
                        }
                    }
                    pub fn SetPrefix<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).28)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for IXmlText {
                    const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                        b"{f931a4cb-308d-4760-a1d5-43b67450ac7e}",
                    );
                }
                impl ::std::convert::From<IXmlText> for ::windows::IUnknown {
                    fn from(value: IXmlText) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&IXmlText> for ::windows::IUnknown {
                    fn from(value: &IXmlText) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IXmlText {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IXmlText {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<IXmlText> for ::windows::IInspectable {
                    fn from(value: IXmlText) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&IXmlText> for ::windows::IInspectable {
                    fn from(value: &IXmlText) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for IXmlText {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IXmlText {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<IXmlText> for IXmlCharacterData {
                    type Error = ::windows::Error;
                    fn try_from(value: IXmlText) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&IXmlText> for IXmlCharacterData {
                    type Error = ::windows::Error;
                    fn try_from(value: &IXmlText) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlCharacterData> for IXmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlCharacterData> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlCharacterData> for &IXmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlCharacterData> {
                        ::std::convert::TryInto::<IXmlCharacterData>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<IXmlText> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: IXmlText) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&IXmlText> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: &IXmlText) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for IXmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for &IXmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::std::convert::TryInto::<IXmlNode>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<IXmlText> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: IXmlText) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&IXmlText> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: &IXmlText) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for IXmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for &IXmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<IXmlText> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: IXmlText) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&IXmlText> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: &IXmlText) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for IXmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for &IXmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXmlText_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        count: *mut u32,
                        values: *mut *mut ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        value: *mut i32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        offset: u32,
                        result__: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct NodeType(pub i32);
                impl NodeType {
                    pub const Invalid: NodeType = NodeType(0i32);
                    pub const ElementNode: NodeType = NodeType(1i32);
                    pub const AttributeNode: NodeType = NodeType(2i32);
                    pub const TextNode: NodeType = NodeType(3i32);
                    pub const DataSectionNode: NodeType = NodeType(4i32);
                    pub const EntityReferenceNode: NodeType = NodeType(5i32);
                    pub const EntityNode: NodeType = NodeType(6i32);
                    pub const ProcessingInstructionNode: NodeType = NodeType(7i32);
                    pub const CommentNode: NodeType = NodeType(8i32);
                    pub const DocumentNode: NodeType = NodeType(9i32);
                    pub const DocumentTypeNode: NodeType = NodeType(10i32);
                    pub const DocumentFragmentNode: NodeType = NodeType(11i32);
                    pub const NotationNode: NodeType = NodeType(12i32);
                }
                impl ::std::convert::From<i32> for NodeType {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for NodeType {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                unsafe impl ::windows::RuntimeType for NodeType {
                    const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                        b"enum(Windows.Data.Xml.Dom.NodeType;i4)",
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct XmlAttribute(::windows::IInspectable);
                impl XmlAttribute {
                    pub fn Name(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn Specified(&self) -> ::windows::Result<bool> {
                        let this = self;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn Value(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetValue<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeValue(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SetNodeValue<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeType(&self) -> ::windows::Result<NodeType> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <NodeType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<NodeType>(result__)
                        }
                    }
                    pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn ParentNode(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ChildNodes(&self) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn FirstChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LastChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn PreviousSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NextSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Attributes(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn HasChildNodes(&self) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn OwnerDocument(&self) -> ::windows::Result<XmlDocument> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlDocument as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocument>(result__)
                        }
                    }
                    pub fn InsertBefore<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ReplaceChild<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        childnode: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                childnode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn AppendChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        newchild: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn CloneNode(&self, deep: bool) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).23)(
                                ::std::mem::transmute_copy(this),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NamespaceUri(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).24)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn LocalName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).25)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Prefix(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).26)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Normalize(&self) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).27)(::std::mem::transmute_copy(
                                this,
                            ))
                            .ok()
                        }
                    }
                    pub fn SetPrefix<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).28)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for XmlAttribute {
                    const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Data.Xml.Dom.XmlAttribute;{ac144aa4-b4f1-4db6-b206-8a22c308db0a})" ) ;
                }
                unsafe impl ::windows::Interface for XmlAttribute {
                    type Vtable = IXmlAttribute_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        2887010980,
                        46321,
                        19894,
                        [178, 6, 138, 34, 195, 8, 219, 10],
                    );
                }
                impl ::windows::RuntimeName for XmlAttribute {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlAttribute";
                }
                impl ::std::convert::From<XmlAttribute> for ::windows::IUnknown {
                    fn from(value: XmlAttribute) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&XmlAttribute> for ::windows::IUnknown {
                    fn from(value: &XmlAttribute) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for XmlAttribute {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &XmlAttribute {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<XmlAttribute> for ::windows::IInspectable {
                    fn from(value: XmlAttribute) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&XmlAttribute> for ::windows::IInspectable {
                    fn from(value: &XmlAttribute) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for XmlAttribute {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a XmlAttribute {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<XmlAttribute> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlAttribute) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlAttribute> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlAttribute) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for XmlAttribute {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for &XmlAttribute {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::std::convert::TryInto::<IXmlNode>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlAttribute> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlAttribute) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlAttribute> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlAttribute) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for XmlAttribute {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for &XmlAttribute {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlAttribute> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlAttribute) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlAttribute> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlAttribute) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for XmlAttribute {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for &XmlAttribute {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                unsafe impl ::std::marker::Send for XmlAttribute {}
                unsafe impl ::std::marker::Sync for XmlAttribute {}
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct XmlCDataSection(::windows::IInspectable);
                impl XmlCDataSection {
                    pub fn Data(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetData<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn Length(&self) -> ::windows::Result<u32> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<u32>(result__)
                        }
                    }
                    pub fn SubstringData(
                        &self,
                        offset: u32,
                        count: u32,
                    ) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                count,
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn AppendData<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        data: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                data.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn InsertData<'a, Param1: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        offset: u32,
                        data: Param1,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                data.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                count,
                            )
                            .ok()
                        }
                    }
                    pub fn ReplaceData<'a, Param2: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        offset: u32,
                        count: u32,
                        data: Param2,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                count,
                                data.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeValue(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SetNodeValue<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeType(&self) -> ::windows::Result<NodeType> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <NodeType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<NodeType>(result__)
                        }
                    }
                    pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn ParentNode(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ChildNodes(&self) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn FirstChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LastChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn PreviousSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NextSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Attributes(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn HasChildNodes(&self) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn OwnerDocument(&self) -> ::windows::Result<XmlDocument> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlDocument as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocument>(result__)
                        }
                    }
                    pub fn InsertBefore<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ReplaceChild<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        childnode: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                childnode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn AppendChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        newchild: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn CloneNode(&self, deep: bool) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).23)(
                                ::std::mem::transmute_copy(this),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NamespaceUri(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).24)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn LocalName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).25)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Prefix(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).26)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Normalize(&self) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).27)(::std::mem::transmute_copy(
                                this,
                            ))
                            .ok()
                        }
                    }
                    pub fn SetPrefix<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).28)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SplitText(&self, offset: u32) -> ::windows::Result<IXmlText> {
                        let this = &::windows::Interface::cast::<IXmlText>(self)?;
                        unsafe {
                            let mut result__: <IXmlText as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                &mut result__,
                            )
                            .from_abi::<IXmlText>(result__)
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for XmlCDataSection {
                    const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Data.Xml.Dom.XmlCDataSection;{4d04b46f-c8bd-45b4-8899-0400d7c2c60f})" ) ;
                }
                unsafe impl ::windows::Interface for XmlCDataSection {
                    type Vtable = IXmlCDataSection_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1292153967,
                        51389,
                        17844,
                        [136, 153, 4, 0, 215, 194, 198, 15],
                    );
                }
                impl ::windows::RuntimeName for XmlCDataSection {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlCDataSection";
                }
                impl ::std::convert::From<XmlCDataSection> for ::windows::IUnknown {
                    fn from(value: XmlCDataSection) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&XmlCDataSection> for ::windows::IUnknown {
                    fn from(value: &XmlCDataSection) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for XmlCDataSection {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &XmlCDataSection {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<XmlCDataSection> for ::windows::IInspectable {
                    fn from(value: XmlCDataSection) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&XmlCDataSection> for ::windows::IInspectable {
                    fn from(value: &XmlCDataSection) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for XmlCDataSection {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a XmlCDataSection {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<XmlCDataSection> for IXmlCharacterData {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlCDataSection) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlCDataSection> for IXmlCharacterData {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlCDataSection) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlCharacterData> for XmlCDataSection {
                    fn into_param(self) -> ::windows::Param<'a, IXmlCharacterData> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlCharacterData> for &XmlCDataSection {
                    fn into_param(self) -> ::windows::Param<'a, IXmlCharacterData> {
                        ::std::convert::TryInto::<IXmlCharacterData>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlCDataSection> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlCDataSection) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlCDataSection> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlCDataSection) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for XmlCDataSection {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for &XmlCDataSection {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::std::convert::TryInto::<IXmlNode>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlCDataSection> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlCDataSection) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlCDataSection> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlCDataSection) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for XmlCDataSection {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for &XmlCDataSection {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlCDataSection> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlCDataSection) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlCDataSection> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlCDataSection) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for XmlCDataSection {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for &XmlCDataSection {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlCDataSection> for IXmlText {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlCDataSection) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlCDataSection> for IXmlText {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlCDataSection) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlText> for XmlCDataSection {
                    fn into_param(self) -> ::windows::Param<'a, IXmlText> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlText> for &XmlCDataSection {
                    fn into_param(self) -> ::windows::Param<'a, IXmlText> {
                        ::std::convert::TryInto::<IXmlText>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                unsafe impl ::std::marker::Send for XmlCDataSection {}
                unsafe impl ::std::marker::Sync for XmlCDataSection {}
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct XmlComment(::windows::IInspectable);
                impl XmlComment {
                    pub fn Data(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetData<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn Length(&self) -> ::windows::Result<u32> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<u32>(result__)
                        }
                    }
                    pub fn SubstringData(
                        &self,
                        offset: u32,
                        count: u32,
                    ) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                count,
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn AppendData<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        data: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                data.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn InsertData<'a, Param1: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        offset: u32,
                        data: Param1,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                data.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                count,
                            )
                            .ok()
                        }
                    }
                    pub fn ReplaceData<'a, Param2: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        offset: u32,
                        count: u32,
                        data: Param2,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                count,
                                data.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeValue(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SetNodeValue<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeType(&self) -> ::windows::Result<NodeType> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <NodeType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<NodeType>(result__)
                        }
                    }
                    pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn ParentNode(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ChildNodes(&self) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn FirstChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LastChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn PreviousSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NextSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Attributes(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn HasChildNodes(&self) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn OwnerDocument(&self) -> ::windows::Result<XmlDocument> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlDocument as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocument>(result__)
                        }
                    }
                    pub fn InsertBefore<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ReplaceChild<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        childnode: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                childnode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn AppendChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        newchild: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn CloneNode(&self, deep: bool) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).23)(
                                ::std::mem::transmute_copy(this),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NamespaceUri(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).24)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn LocalName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).25)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Prefix(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).26)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Normalize(&self) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).27)(::std::mem::transmute_copy(
                                this,
                            ))
                            .ok()
                        }
                    }
                    pub fn SetPrefix<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).28)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for XmlComment {
                    const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Data.Xml.Dom.XmlComment;{bca474d5-b61f-4611-9cac-2e92e3476d47})" ) ;
                }
                unsafe impl ::windows::Interface for XmlComment {
                    type Vtable = IXmlComment_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        3164894421,
                        46623,
                        17937,
                        [156, 172, 46, 146, 227, 71, 109, 71],
                    );
                }
                impl ::windows::RuntimeName for XmlComment {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlComment";
                }
                impl ::std::convert::From<XmlComment> for ::windows::IUnknown {
                    fn from(value: XmlComment) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&XmlComment> for ::windows::IUnknown {
                    fn from(value: &XmlComment) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for XmlComment {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &XmlComment {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<XmlComment> for ::windows::IInspectable {
                    fn from(value: XmlComment) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&XmlComment> for ::windows::IInspectable {
                    fn from(value: &XmlComment) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for XmlComment {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a XmlComment {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<XmlComment> for IXmlCharacterData {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlComment) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlComment> for IXmlCharacterData {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlComment) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlCharacterData> for XmlComment {
                    fn into_param(self) -> ::windows::Param<'a, IXmlCharacterData> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlCharacterData> for &XmlComment {
                    fn into_param(self) -> ::windows::Param<'a, IXmlCharacterData> {
                        ::std::convert::TryInto::<IXmlCharacterData>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlComment> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlComment) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlComment> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlComment) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for XmlComment {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for &XmlComment {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::std::convert::TryInto::<IXmlNode>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlComment> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlComment) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlComment> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlComment) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for XmlComment {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for &XmlComment {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlComment> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlComment) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlComment> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlComment) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for XmlComment {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for &XmlComment {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                unsafe impl ::std::marker::Send for XmlComment {}
                unsafe impl ::std::marker::Sync for XmlComment {}
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct XmlDocument(::windows::IInspectable);
                impl XmlDocument {
                    pub fn new() -> ::windows::Result<Self> {
                        Self::IActivationFactory(|f| f.activate_instance::<Self>())
                    }
                    fn IActivationFactory<
                        R,
                        F: FnOnce(&::windows::IActivationFactory) -> ::windows::Result<R>,
                    >(
                        callback: F,
                    ) -> ::windows::Result<R> {
                        static mut SHARED: ::windows::FactoryCache<
                            XmlDocument,
                            ::windows::IActivationFactory,
                        > = ::windows::FactoryCache::new();
                        unsafe { SHARED.call(callback) }
                    }
                    pub fn Doctype(&self) -> ::windows::Result<XmlDocumentType> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlDocumentType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocumentType>(result__)
                        }
                    }
                    pub fn Implementation(&self) -> ::windows::Result<XmlDomImplementation> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlDomImplementation as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDomImplementation>(result__)
                        }
                    }
                    pub fn DocumentElement(&self) -> ::windows::Result<XmlElement> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlElement as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlElement>(result__)
                        }
                    }
                    pub fn CreateElement<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        tagname: Param0,
                    ) -> ::windows::Result<XmlElement> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlElement as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                tagname.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlElement>(result__)
                        }
                    }
                    pub fn CreateDocumentFragment(&self) -> ::windows::Result<XmlDocumentFragment> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlDocumentFragment as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocumentFragment>(result__)
                        }
                    }
                    pub fn CreateTextNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        data: Param0,
                    ) -> ::windows::Result<XmlText> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlText as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                data.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlText>(result__)
                        }
                    }
                    pub fn CreateComment<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        data: Param0,
                    ) -> ::windows::Result<XmlComment> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlComment as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                data.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlComment>(result__)
                        }
                    }
                    pub fn CreateProcessingInstruction<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        target: Param0,
                        data: Param1,
                    ) -> ::windows::Result<XmlProcessingInstruction> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlProcessingInstruction as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                target.into_param().abi(),
                                data.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlProcessingInstruction>(result__)
                        }
                    }
                    pub fn CreateAttribute<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        name: Param0,
                    ) -> ::windows::Result<XmlAttribute> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlAttribute as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                name.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlAttribute>(result__)
                        }
                    }
                    pub fn CreateEntityReference<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        name: Param0,
                    ) -> ::windows::Result<XmlEntityReference> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlEntityReference as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                name.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlEntityReference>(result__)
                        }
                    }
                    pub fn GetElementsByTagName<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        tagname: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                tagname.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn CreateCDataSection<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        data: Param0,
                    ) -> ::windows::Result<XmlCDataSection> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlCDataSection as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                data.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlCDataSection>(result__)
                        }
                    }
                    pub fn DocumentUri(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn CreateAttributeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                        Param1: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        namespaceuri: Param0,
                        qualifiedname: Param1,
                    ) -> ::windows::Result<XmlAttribute> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlAttribute as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                namespaceuri.into_param().abi(),
                                qualifiedname.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlAttribute>(result__)
                        }
                    }
                    pub fn CreateElementNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                        Param1: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        namespaceuri: Param0,
                        qualifiedname: Param1,
                    ) -> ::windows::Result<XmlElement> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlElement as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                namespaceuri.into_param().abi(),
                                qualifiedname.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlElement>(result__)
                        }
                    }
                    pub fn GetElementById<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        elementid: Param0,
                    ) -> ::windows::Result<XmlElement> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlElement as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                elementid.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlElement>(result__)
                        }
                    }
                    pub fn ImportNode<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        node: Param0,
                        deep: bool,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                node.into_param().abi(),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LoadXml<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xml: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlDocumentIO>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xml.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn LoadXmlWithSettings<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, XmlLoadSettings>,
                    >(
                        &self,
                        xml: Param0,
                        loadsettings: Param1,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlDocumentIO>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xml.into_param().abi(),
                                loadsettings.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SaveToFileAsync<
                        'a,
                        Param0: ::windows::IntoParam<'a, super::super::super::Storage::IStorageFile>,
                    >(
                        &self,
                        file: Param0,
                    ) -> ::windows::Result<super::super::super::Foundation::IAsyncAction>
                    {
                        let this = &::windows::Interface::cast::<IXmlDocumentIO>(self)?;
                        unsafe {
                            let mut result__ : < super::super::super::Foundation:: IAsyncAction as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                file.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
                        }
                    }
                    pub fn LoadXmlFromBuffer<
                        'a,
                        Param0: ::windows::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>,
                    >(
                        &self,
                        buffer: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlDocumentIO2>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                buffer.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn LoadXmlFromBufferWithSettings<
                        'a,
                        Param0: ::windows::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>,
                        Param1: ::windows::IntoParam<'a, XmlLoadSettings>,
                    >(
                        &self,
                        buffer: Param0,
                        loadsettings: Param1,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlDocumentIO2>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                buffer.into_param().abi(),
                                loadsettings.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeValue(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SetNodeValue<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeType(&self) -> ::windows::Result<NodeType> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <NodeType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<NodeType>(result__)
                        }
                    }
                    pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn ParentNode(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ChildNodes(&self) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn FirstChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LastChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn PreviousSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NextSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Attributes(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn HasChildNodes(&self) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn OwnerDocument(&self) -> ::windows::Result<XmlDocument> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlDocument as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocument>(result__)
                        }
                    }
                    pub fn InsertBefore<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ReplaceChild<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        childnode: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                childnode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn AppendChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        newchild: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn CloneNode(&self, deep: bool) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).23)(
                                ::std::mem::transmute_copy(this),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NamespaceUri(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).24)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn LocalName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).25)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Prefix(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).26)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Normalize(&self) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).27)(::std::mem::transmute_copy(
                                this,
                            ))
                            .ok()
                        }
                    }
                    pub fn SetPrefix<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).28)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn LoadFromUriAsync<
                        'a,
                        Param0: ::windows::IntoParam<'a, super::super::super::Foundation::Uri>,
                    >(
                        uri: Param0,
                    ) -> ::windows::Result<
                        super::super::super::Foundation::IAsyncOperation<XmlDocument>,
                    > {
                        Self::IXmlDocumentStatics(|this| unsafe {
                            let mut result__: <super::super::super::Foundation::IAsyncOperation<
                                XmlDocument,
                            > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            ( :: windows :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , uri . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::super::Foundation:: IAsyncOperation :: < XmlDocument > > ( result__ )
                        })
                    }
                    pub fn LoadFromUriWithSettingsAsync<
                        'a,
                        Param0: ::windows::IntoParam<'a, super::super::super::Foundation::Uri>,
                        Param1: ::windows::IntoParam<'a, XmlLoadSettings>,
                    >(
                        uri: Param0,
                        loadsettings: Param1,
                    ) -> ::windows::Result<
                        super::super::super::Foundation::IAsyncOperation<XmlDocument>,
                    > {
                        Self::IXmlDocumentStatics(|this| unsafe {
                            let mut result__: <super::super::super::Foundation::IAsyncOperation<
                                XmlDocument,
                            > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            ( :: windows :: Interface :: vtable ( this ) .7 ) ( :: std :: mem :: transmute_copy ( this ) , uri . into_param ( ) . abi ( ) , loadsettings . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::super::Foundation:: IAsyncOperation :: < XmlDocument > > ( result__ )
                        })
                    }
                    pub fn LoadFromFileAsync<
                        'a,
                        Param0: ::windows::IntoParam<'a, super::super::super::Storage::IStorageFile>,
                    >(
                        file: Param0,
                    ) -> ::windows::Result<
                        super::super::super::Foundation::IAsyncOperation<XmlDocument>,
                    > {
                        Self::IXmlDocumentStatics(|this| unsafe {
                            let mut result__: <super::super::super::Foundation::IAsyncOperation<
                                XmlDocument,
                            > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            ( :: windows :: Interface :: vtable ( this ) .8 ) ( :: std :: mem :: transmute_copy ( this ) , file . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::super::Foundation:: IAsyncOperation :: < XmlDocument > > ( result__ )
                        })
                    }
                    pub fn LoadFromFileWithSettingsAsync<
                        'a,
                        Param0: ::windows::IntoParam<'a, super::super::super::Storage::IStorageFile>,
                        Param1: ::windows::IntoParam<'a, XmlLoadSettings>,
                    >(
                        file: Param0,
                        loadsettings: Param1,
                    ) -> ::windows::Result<
                        super::super::super::Foundation::IAsyncOperation<XmlDocument>,
                    > {
                        Self::IXmlDocumentStatics(|this| unsafe {
                            let mut result__: <super::super::super::Foundation::IAsyncOperation<
                                XmlDocument,
                            > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            ( :: windows :: Interface :: vtable ( this ) .9 ) ( :: std :: mem :: transmute_copy ( this ) , file . into_param ( ) . abi ( ) , loadsettings . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::super::Foundation:: IAsyncOperation :: < XmlDocument > > ( result__ )
                        })
                    }
                    pub fn IXmlDocumentStatics<
                        R,
                        F: FnOnce(&IXmlDocumentStatics) -> ::windows::Result<R>,
                    >(
                        callback: F,
                    ) -> ::windows::Result<R> {
                        static mut SHARED: ::windows::FactoryCache<
                            XmlDocument,
                            IXmlDocumentStatics,
                        > = ::windows::FactoryCache::new();
                        unsafe { SHARED.call(callback) }
                    }
                }
                unsafe impl ::windows::RuntimeType for XmlDocument {
                    const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Data.Xml.Dom.XmlDocument;{f7f3a506-1e87-42d6-bcfb-b8c809fa5494})" ) ;
                }
                unsafe impl ::windows::Interface for XmlDocument {
                    type Vtable = IXmlDocument_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        4159939846,
                        7815,
                        17110,
                        [188, 251, 184, 200, 9, 250, 84, 148],
                    );
                }
                impl ::windows::RuntimeName for XmlDocument {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocument";
                }
                impl ::std::convert::From<XmlDocument> for ::windows::IUnknown {
                    fn from(value: XmlDocument) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&XmlDocument> for ::windows::IUnknown {
                    fn from(value: &XmlDocument) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for XmlDocument {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &XmlDocument {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<XmlDocument> for ::windows::IInspectable {
                    fn from(value: XmlDocument) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&XmlDocument> for ::windows::IInspectable {
                    fn from(value: &XmlDocument) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for XmlDocument {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a XmlDocument {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<XmlDocument> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlDocument) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlDocument> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlDocument) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for XmlDocument {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for &XmlDocument {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::std::convert::TryInto::<IXmlNode>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlDocument> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlDocument) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlDocument> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlDocument) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for XmlDocument {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for &XmlDocument {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlDocument> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlDocument) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlDocument> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlDocument) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for XmlDocument {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for &XmlDocument {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                unsafe impl ::std::marker::Send for XmlDocument {}
                unsafe impl ::std::marker::Sync for XmlDocument {}
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct XmlDocumentFragment(::windows::IInspectable);
                impl XmlDocumentFragment {
                    pub fn NodeValue(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SetNodeValue<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeType(&self) -> ::windows::Result<NodeType> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <NodeType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<NodeType>(result__)
                        }
                    }
                    pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn ParentNode(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ChildNodes(&self) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn FirstChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LastChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn PreviousSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NextSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Attributes(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn HasChildNodes(&self) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn OwnerDocument(&self) -> ::windows::Result<XmlDocument> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlDocument as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocument>(result__)
                        }
                    }
                    pub fn InsertBefore<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ReplaceChild<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        childnode: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                childnode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn AppendChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        newchild: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn CloneNode(&self, deep: bool) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).23)(
                                ::std::mem::transmute_copy(this),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NamespaceUri(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).24)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn LocalName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).25)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Prefix(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).26)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Normalize(&self) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).27)(::std::mem::transmute_copy(
                                this,
                            ))
                            .ok()
                        }
                    }
                    pub fn SetPrefix<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).28)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for XmlDocumentFragment {
                    const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Data.Xml.Dom.XmlDocumentFragment;{e2ea6a96-0c21-44a5-8bc9-9e4a262708ec})" ) ;
                }
                unsafe impl ::windows::Interface for XmlDocumentFragment {
                    type Vtable = IXmlDocumentFragment_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        3807013526,
                        3105,
                        17573,
                        [139, 201, 158, 74, 38, 39, 8, 236],
                    );
                }
                impl ::windows::RuntimeName for XmlDocumentFragment {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentFragment";
                }
                impl ::std::convert::From<XmlDocumentFragment> for ::windows::IUnknown {
                    fn from(value: XmlDocumentFragment) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&XmlDocumentFragment> for ::windows::IUnknown {
                    fn from(value: &XmlDocumentFragment) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for XmlDocumentFragment {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &XmlDocumentFragment {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<XmlDocumentFragment> for ::windows::IInspectable {
                    fn from(value: XmlDocumentFragment) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&XmlDocumentFragment> for ::windows::IInspectable {
                    fn from(value: &XmlDocumentFragment) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for XmlDocumentFragment {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a XmlDocumentFragment {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<XmlDocumentFragment> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlDocumentFragment) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlDocumentFragment> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlDocumentFragment) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for XmlDocumentFragment {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for &XmlDocumentFragment {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::std::convert::TryInto::<IXmlNode>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlDocumentFragment> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlDocumentFragment) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlDocumentFragment> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlDocumentFragment) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for XmlDocumentFragment {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for &XmlDocumentFragment {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlDocumentFragment> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlDocumentFragment) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlDocumentFragment> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlDocumentFragment) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for XmlDocumentFragment {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for &XmlDocumentFragment {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                unsafe impl ::std::marker::Send for XmlDocumentFragment {}
                unsafe impl ::std::marker::Sync for XmlDocumentFragment {}
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct XmlDocumentType(::windows::IInspectable);
                impl XmlDocumentType {
                    pub fn Name(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn Entities(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn Notations(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn NodeValue(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SetNodeValue<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeType(&self) -> ::windows::Result<NodeType> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <NodeType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<NodeType>(result__)
                        }
                    }
                    pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn ParentNode(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ChildNodes(&self) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn FirstChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LastChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn PreviousSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NextSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Attributes(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn HasChildNodes(&self) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn OwnerDocument(&self) -> ::windows::Result<XmlDocument> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlDocument as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocument>(result__)
                        }
                    }
                    pub fn InsertBefore<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ReplaceChild<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        childnode: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                childnode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn AppendChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        newchild: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn CloneNode(&self, deep: bool) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).23)(
                                ::std::mem::transmute_copy(this),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NamespaceUri(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).24)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn LocalName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).25)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Prefix(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).26)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Normalize(&self) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).27)(::std::mem::transmute_copy(
                                this,
                            ))
                            .ok()
                        }
                    }
                    pub fn SetPrefix<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).28)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for XmlDocumentType {
                    const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Data.Xml.Dom.XmlDocumentType;{f7342425-9781-4964-8e94-9b1c6dfc9bc7})" ) ;
                }
                unsafe impl ::windows::Interface for XmlDocumentType {
                    type Vtable = IXmlDocumentType_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        4147389477,
                        38785,
                        18788,
                        [142, 148, 155, 28, 109, 252, 155, 199],
                    );
                }
                impl ::windows::RuntimeName for XmlDocumentType {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentType";
                }
                impl ::std::convert::From<XmlDocumentType> for ::windows::IUnknown {
                    fn from(value: XmlDocumentType) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&XmlDocumentType> for ::windows::IUnknown {
                    fn from(value: &XmlDocumentType) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for XmlDocumentType {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &XmlDocumentType {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<XmlDocumentType> for ::windows::IInspectable {
                    fn from(value: XmlDocumentType) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&XmlDocumentType> for ::windows::IInspectable {
                    fn from(value: &XmlDocumentType) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for XmlDocumentType {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a XmlDocumentType {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<XmlDocumentType> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlDocumentType) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlDocumentType> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlDocumentType) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for XmlDocumentType {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for &XmlDocumentType {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::std::convert::TryInto::<IXmlNode>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlDocumentType> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlDocumentType) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlDocumentType> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlDocumentType) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for XmlDocumentType {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for &XmlDocumentType {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlDocumentType> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlDocumentType) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlDocumentType> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlDocumentType) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for XmlDocumentType {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for &XmlDocumentType {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                unsafe impl ::std::marker::Send for XmlDocumentType {}
                unsafe impl ::std::marker::Sync for XmlDocumentType {}
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct XmlDomImplementation(::windows::IInspectable);
                impl XmlDomImplementation {
                    pub fn HasFeature<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        feature: Param0,
                        version: Param1,
                    ) -> ::windows::Result<bool> {
                        let this = self;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                feature.into_param().abi(),
                                version.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for XmlDomImplementation {
                    const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Data.Xml.Dom.XmlDomImplementation;{6de58132-f11d-4fbb-8cc6-583cba93112f})" ) ;
                }
                unsafe impl ::windows::Interface for XmlDomImplementation {
                    type Vtable = IXmlDomImplementation_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1843757362,
                        61725,
                        20411,
                        [140, 198, 88, 60, 186, 147, 17, 47],
                    );
                }
                impl ::windows::RuntimeName for XmlDomImplementation {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDomImplementation";
                }
                impl ::std::convert::From<XmlDomImplementation> for ::windows::IUnknown {
                    fn from(value: XmlDomImplementation) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&XmlDomImplementation> for ::windows::IUnknown {
                    fn from(value: &XmlDomImplementation) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for XmlDomImplementation {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &XmlDomImplementation {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<XmlDomImplementation> for ::windows::IInspectable {
                    fn from(value: XmlDomImplementation) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&XmlDomImplementation> for ::windows::IInspectable {
                    fn from(value: &XmlDomImplementation) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for XmlDomImplementation {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a XmlDomImplementation {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                unsafe impl ::std::marker::Send for XmlDomImplementation {}
                unsafe impl ::std::marker::Sync for XmlDomImplementation {}
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct XmlElement(::windows::IInspectable);
                impl XmlElement {
                    pub fn TagName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn GetAttribute<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        attributename: Param0,
                    ) -> ::windows::Result<::windows::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                attributename.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetAttribute<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        attributename: Param0,
                        attributevalue: Param1,
                    ) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                attributename.into_param().abi(),
                                attributevalue.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn RemoveAttribute<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        attributename: Param0,
                    ) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                attributename.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn GetAttributeNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        attributename: Param0,
                    ) -> ::windows::Result<XmlAttribute> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlAttribute as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                attributename.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlAttribute>(result__)
                        }
                    }
                    pub fn SetAttributeNode<'a, Param0: ::windows::IntoParam<'a, XmlAttribute>>(
                        &self,
                        newattribute: Param0,
                    ) -> ::windows::Result<XmlAttribute> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlAttribute as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                newattribute.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlAttribute>(result__)
                        }
                    }
                    pub fn RemoveAttributeNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, XmlAttribute>,
                    >(
                        &self,
                        attributenode: Param0,
                    ) -> ::windows::Result<XmlAttribute> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlAttribute as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                attributenode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlAttribute>(result__)
                        }
                    }
                    pub fn GetElementsByTagName<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        tagname: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                tagname.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SetAttributeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                        Param1: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param2: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        namespaceuri: Param0,
                        qualifiedname: Param1,
                        value: Param2,
                    ) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                namespaceuri.into_param().abi(),
                                qualifiedname.into_param().abi(),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn GetAttributeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                        Param1: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        namespaceuri: Param0,
                        localname: Param1,
                    ) -> ::windows::Result<::windows::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                namespaceuri.into_param().abi(),
                                localname.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn RemoveAttributeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                        Param1: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        namespaceuri: Param0,
                        localname: Param1,
                    ) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                namespaceuri.into_param().abi(),
                                localname.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SetAttributeNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, XmlAttribute>,
                    >(
                        &self,
                        newattribute: Param0,
                    ) -> ::windows::Result<XmlAttribute> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlAttribute as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                newattribute.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlAttribute>(result__)
                        }
                    }
                    pub fn GetAttributeNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                        Param1: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        namespaceuri: Param0,
                        localname: Param1,
                    ) -> ::windows::Result<XmlAttribute> {
                        let this = self;
                        unsafe {
                            let mut result__: <XmlAttribute as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                namespaceuri.into_param().abi(),
                                localname.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlAttribute>(result__)
                        }
                    }
                    pub fn NodeValue(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SetNodeValue<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeType(&self) -> ::windows::Result<NodeType> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <NodeType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<NodeType>(result__)
                        }
                    }
                    pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn ParentNode(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ChildNodes(&self) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn FirstChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LastChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn PreviousSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NextSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Attributes(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn HasChildNodes(&self) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn OwnerDocument(&self) -> ::windows::Result<XmlDocument> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlDocument as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocument>(result__)
                        }
                    }
                    pub fn InsertBefore<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ReplaceChild<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        childnode: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                childnode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn AppendChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        newchild: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn CloneNode(&self, deep: bool) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).23)(
                                ::std::mem::transmute_copy(this),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NamespaceUri(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).24)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn LocalName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).25)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Prefix(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).26)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Normalize(&self) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).27)(::std::mem::transmute_copy(
                                this,
                            ))
                            .ok()
                        }
                    }
                    pub fn SetPrefix<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).28)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for XmlElement {
                    const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Data.Xml.Dom.XmlElement;{2dfb8a1f-6b10-4ef8-9f83-efcce8faec37})" ) ;
                }
                unsafe impl ::windows::Interface for XmlElement {
                    type Vtable = IXmlElement_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        771459615,
                        27408,
                        20216,
                        [159, 131, 239, 204, 232, 250, 236, 55],
                    );
                }
                impl ::windows::RuntimeName for XmlElement {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlElement";
                }
                impl ::std::convert::From<XmlElement> for ::windows::IUnknown {
                    fn from(value: XmlElement) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&XmlElement> for ::windows::IUnknown {
                    fn from(value: &XmlElement) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for XmlElement {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &XmlElement {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<XmlElement> for ::windows::IInspectable {
                    fn from(value: XmlElement) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&XmlElement> for ::windows::IInspectable {
                    fn from(value: &XmlElement) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for XmlElement {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a XmlElement {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<XmlElement> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlElement) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlElement> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlElement) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for XmlElement {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for &XmlElement {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::std::convert::TryInto::<IXmlNode>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlElement> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlElement) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlElement> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlElement) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for XmlElement {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for &XmlElement {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlElement> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlElement) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlElement> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlElement) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for XmlElement {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for &XmlElement {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                unsafe impl ::std::marker::Send for XmlElement {}
                unsafe impl ::std::marker::Sync for XmlElement {}
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct XmlEntityReference(::windows::IInspectable);
                impl XmlEntityReference {
                    pub fn NodeValue(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SetNodeValue<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeType(&self) -> ::windows::Result<NodeType> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <NodeType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<NodeType>(result__)
                        }
                    }
                    pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn ParentNode(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ChildNodes(&self) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn FirstChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LastChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn PreviousSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NextSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Attributes(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn HasChildNodes(&self) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn OwnerDocument(&self) -> ::windows::Result<XmlDocument> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlDocument as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocument>(result__)
                        }
                    }
                    pub fn InsertBefore<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ReplaceChild<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        childnode: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                childnode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn AppendChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        newchild: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn CloneNode(&self, deep: bool) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).23)(
                                ::std::mem::transmute_copy(this),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NamespaceUri(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).24)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn LocalName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).25)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Prefix(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).26)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Normalize(&self) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).27)(::std::mem::transmute_copy(
                                this,
                            ))
                            .ok()
                        }
                    }
                    pub fn SetPrefix<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).28)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for XmlEntityReference {
                    const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Data.Xml.Dom.XmlEntityReference;{2e2f47bc-c3d0-4ccf-bb86-0ab8c36a61cf})" ) ;
                }
                unsafe impl ::windows::Interface for XmlEntityReference {
                    type Vtable = IXmlEntityReference_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        774850492,
                        50128,
                        19663,
                        [187, 134, 10, 184, 195, 106, 97, 207],
                    );
                }
                impl ::windows::RuntimeName for XmlEntityReference {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlEntityReference";
                }
                impl ::std::convert::From<XmlEntityReference> for ::windows::IUnknown {
                    fn from(value: XmlEntityReference) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&XmlEntityReference> for ::windows::IUnknown {
                    fn from(value: &XmlEntityReference) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for XmlEntityReference {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &XmlEntityReference {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<XmlEntityReference> for ::windows::IInspectable {
                    fn from(value: XmlEntityReference) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&XmlEntityReference> for ::windows::IInspectable {
                    fn from(value: &XmlEntityReference) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for XmlEntityReference {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a XmlEntityReference {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<XmlEntityReference> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlEntityReference) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlEntityReference> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlEntityReference) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for XmlEntityReference {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for &XmlEntityReference {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::std::convert::TryInto::<IXmlNode>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlEntityReference> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlEntityReference) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlEntityReference> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlEntityReference) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for XmlEntityReference {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for &XmlEntityReference {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlEntityReference> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlEntityReference) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlEntityReference> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlEntityReference) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for XmlEntityReference {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for &XmlEntityReference {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                unsafe impl ::std::marker::Send for XmlEntityReference {}
                unsafe impl ::std::marker::Sync for XmlEntityReference {}
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct XmlLoadSettings(::windows::IInspectable);
                impl XmlLoadSettings {
                    pub fn new() -> ::windows::Result<Self> {
                        Self::IActivationFactory(|f| f.activate_instance::<Self>())
                    }
                    fn IActivationFactory<
                        R,
                        F: FnOnce(&::windows::IActivationFactory) -> ::windows::Result<R>,
                    >(
                        callback: F,
                    ) -> ::windows::Result<R> {
                        static mut SHARED: ::windows::FactoryCache<
                            XmlLoadSettings,
                            ::windows::IActivationFactory,
                        > = ::windows::FactoryCache::new();
                        unsafe { SHARED.call(callback) }
                    }
                    pub fn MaxElementDepth(&self) -> ::windows::Result<u32> {
                        let this = self;
                        unsafe {
                            let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<u32>(result__)
                        }
                    }
                    pub fn SetMaxElementDepth(&self, value: u32) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn ProhibitDtd(&self) -> ::windows::Result<bool> {
                        let this = self;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn SetProhibitDtd(&self, value: bool) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn ResolveExternals(&self) -> ::windows::Result<bool> {
                        let this = self;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn SetResolveExternals(&self, value: bool) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn ValidateOnParse(&self) -> ::windows::Result<bool> {
                        let this = self;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn SetValidateOnParse(&self, value: bool) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn ElementContentWhiteSpace(&self) -> ::windows::Result<bool> {
                        let this = self;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn SetElementContentWhiteSpace(
                        &self,
                        value: bool,
                    ) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                value,
                            )
                            .ok()
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for XmlLoadSettings {
                    const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Data.Xml.Dom.XmlLoadSettings;{58aa07a8-fed6-46f7-b4c5-fb1ba72108d6})" ) ;
                }
                unsafe impl ::windows::Interface for XmlLoadSettings {
                    type Vtable = IXmlLoadSettings_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1487538088,
                        65238,
                        18167,
                        [180, 197, 251, 27, 167, 33, 8, 214],
                    );
                }
                impl ::windows::RuntimeName for XmlLoadSettings {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlLoadSettings";
                }
                impl ::std::convert::From<XmlLoadSettings> for ::windows::IUnknown {
                    fn from(value: XmlLoadSettings) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&XmlLoadSettings> for ::windows::IUnknown {
                    fn from(value: &XmlLoadSettings) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for XmlLoadSettings {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &XmlLoadSettings {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<XmlLoadSettings> for ::windows::IInspectable {
                    fn from(value: XmlLoadSettings) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&XmlLoadSettings> for ::windows::IInspectable {
                    fn from(value: &XmlLoadSettings) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for XmlLoadSettings {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a XmlLoadSettings {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                unsafe impl ::std::marker::Send for XmlLoadSettings {}
                unsafe impl ::std::marker::Sync for XmlLoadSettings {}
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct XmlNamedNodeMap(::windows::IInspectable);
                impl XmlNamedNodeMap {
                    pub fn Length(&self) -> ::windows::Result<u32> {
                        let this = self;
                        unsafe {
                            let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<u32>(result__)
                        }
                    }
                    pub fn Item(&self, index: u32) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                index,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn GetNamedItem<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        name: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                name.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SetNamedItem<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        node: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                node.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveNamedItem<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        name: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                name.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn GetNamedItemNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                        Param1: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        namespaceuri: Param0,
                        name: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                namespaceuri.into_param().abi(),
                                name.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveNamedItemNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                        Param1: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        namespaceuri: Param0,
                        name: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                namespaceuri.into_param().abi(),
                                name.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SetNamedItemNS<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        node: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                node.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn First(
                        &self,
                    ) -> ::windows::Result<
                        super::super::super::Foundation::Collections::IIterator<IXmlNode>,
                    > {
                        let this = &::windows::Interface::cast::<
                            super::super::super::Foundation::Collections::IIterable<IXmlNode>,
                        >(self)?;
                        unsafe {
                            let mut result__ : < super::super::super::Foundation::Collections:: IIterator :: < IXmlNode > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                            ( :: windows :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::super::Foundation::Collections:: IIterator :: < IXmlNode > > ( result__ )
                        }
                    }
                    pub fn GetAt(&self, index: u32) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<
                            super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                        >(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                index,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Size(&self) -> ::windows::Result<u32> {
                        let this = &::windows::Interface::cast::<
                            super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                        >(self)?;
                        unsafe {
                            let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<u32>(result__)
                        }
                    }
                    pub fn IndexOf<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        value: Param0,
                        index: &mut u32,
                    ) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<
                            super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                        >(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                                index,
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn GetMany(
                        &self,
                        startindex: u32,
                        items: &mut [<IXmlNode as ::windows::Abi>::DefaultType],
                    ) -> ::windows::Result<u32> {
                        let this = &::windows::Interface::cast::<
                            super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                        >(self)?;
                        unsafe {
                            let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                startindex,
                                items.len() as u32,
                                ::std::mem::transmute_copy(&items),
                                &mut result__,
                            )
                            .from_abi::<u32>(result__)
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for XmlNamedNodeMap {
                    const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Data.Xml.Dom.XmlNamedNodeMap;{b3a69eb0-aab0-4b82-a6fa-b1453f7c021b})" ) ;
                }
                unsafe impl ::windows::Interface for XmlNamedNodeMap {
                    type Vtable = IXmlNamedNodeMap_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        3014041264,
                        43696,
                        19330,
                        [166, 250, 177, 69, 63, 124, 2, 27],
                    );
                }
                impl ::windows::RuntimeName for XmlNamedNodeMap {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlNamedNodeMap";
                }
                impl ::std::convert::From<XmlNamedNodeMap> for ::windows::IUnknown {
                    fn from(value: XmlNamedNodeMap) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&XmlNamedNodeMap> for ::windows::IUnknown {
                    fn from(value: &XmlNamedNodeMap) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for XmlNamedNodeMap {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &XmlNamedNodeMap {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<XmlNamedNodeMap> for ::windows::IInspectable {
                    fn from(value: XmlNamedNodeMap) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&XmlNamedNodeMap> for ::windows::IInspectable {
                    fn from(value: &XmlNamedNodeMap) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for XmlNamedNodeMap {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a XmlNamedNodeMap {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<XmlNamedNodeMap>
                    for super::super::super::Foundation::Collections::IIterable<IXmlNode>
                {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlNamedNodeMap) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlNamedNodeMap>
                    for super::super::super::Foundation::Collections::IIterable<IXmlNode>
                {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlNamedNodeMap) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a>
                    ::windows::IntoParam<
                        'a,
                        super::super::super::Foundation::Collections::IIterable<IXmlNode>,
                    > for XmlNamedNodeMap
                {
                    fn into_param(
                        self,
                    ) -> ::windows::Param<
                        'a,
                        super::super::super::Foundation::Collections::IIterable<IXmlNode>,
                    > {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a>
                    ::windows::IntoParam<
                        'a,
                        super::super::super::Foundation::Collections::IIterable<IXmlNode>,
                    > for &XmlNamedNodeMap
                {
                    fn into_param(
                        self,
                    ) -> ::windows::Param<
                        'a,
                        super::super::super::Foundation::Collections::IIterable<IXmlNode>,
                    > {
                        ::std::convert::TryInto::<
                            super::super::super::Foundation::Collections::IIterable<IXmlNode>,
                        >::try_into(self)
                        .map(::windows::Param::Owned)
                        .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlNamedNodeMap>
                    for super::super::super::Foundation::Collections::IVectorView<IXmlNode>
                {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlNamedNodeMap) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlNamedNodeMap>
                    for super::super::super::Foundation::Collections::IVectorView<IXmlNode>
                {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlNamedNodeMap) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a>
                    ::windows::IntoParam<
                        'a,
                        super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                    > for XmlNamedNodeMap
                {
                    fn into_param(
                        self,
                    ) -> ::windows::Param<
                        'a,
                        super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                    > {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a>
                    ::windows::IntoParam<
                        'a,
                        super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                    > for &XmlNamedNodeMap
                {
                    fn into_param(
                        self,
                    ) -> ::windows::Param<
                        'a,
                        super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                    > {
                        ::std::convert::TryInto::<
                            super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                        >::try_into(self)
                        .map(::windows::Param::Owned)
                        .unwrap_or(::windows::Param::None)
                    }
                }
                unsafe impl ::std::marker::Send for XmlNamedNodeMap {}
                unsafe impl ::std::marker::Sync for XmlNamedNodeMap {}
                impl ::std::iter::IntoIterator for XmlNamedNodeMap {
                    type Item = IXmlNode;
                    type IntoIter =
                        super::super::super::Foundation::Collections::VectorViewIterator<
                            Self::Item,
                        >;
                    fn into_iter(self) -> Self::IntoIter {
                        ::std::iter::IntoIterator::into_iter(&self)
                    }
                }
                impl ::std::iter::IntoIterator for &XmlNamedNodeMap {
                    type Item = IXmlNode;
                    type IntoIter =
                        super::super::super::Foundation::Collections::VectorViewIterator<
                            Self::Item,
                        >;
                    fn into_iter(self) -> Self::IntoIter {
                        super::super::super::Foundation::Collections::VectorViewIterator::new(
                            ::std::convert::TryInto::try_into(self).ok(),
                        )
                    }
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct XmlNodeList(::windows::IInspectable);
                impl XmlNodeList {
                    pub fn Length(&self) -> ::windows::Result<u32> {
                        let this = self;
                        unsafe {
                            let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<u32>(result__)
                        }
                    }
                    pub fn Item(&self, index: u32) -> ::windows::Result<IXmlNode> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                index,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn First(
                        &self,
                    ) -> ::windows::Result<
                        super::super::super::Foundation::Collections::IIterator<IXmlNode>,
                    > {
                        let this = &::windows::Interface::cast::<
                            super::super::super::Foundation::Collections::IIterable<IXmlNode>,
                        >(self)?;
                        unsafe {
                            let mut result__ : < super::super::super::Foundation::Collections:: IIterator :: < IXmlNode > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                            ( :: windows :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::super::Foundation::Collections:: IIterator :: < IXmlNode > > ( result__ )
                        }
                    }
                    pub fn GetAt(&self, index: u32) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<
                            super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                        >(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                index,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Size(&self) -> ::windows::Result<u32> {
                        let this = &::windows::Interface::cast::<
                            super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                        >(self)?;
                        unsafe {
                            let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<u32>(result__)
                        }
                    }
                    pub fn IndexOf<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        value: Param0,
                        index: &mut u32,
                    ) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<
                            super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                        >(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                                index,
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn GetMany(
                        &self,
                        startindex: u32,
                        items: &mut [<IXmlNode as ::windows::Abi>::DefaultType],
                    ) -> ::windows::Result<u32> {
                        let this = &::windows::Interface::cast::<
                            super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                        >(self)?;
                        unsafe {
                            let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                startindex,
                                items.len() as u32,
                                ::std::mem::transmute_copy(&items),
                                &mut result__,
                            )
                            .from_abi::<u32>(result__)
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for XmlNodeList {
                    const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Data.Xml.Dom.XmlNodeList;{8c60ad77-83a4-4ec1-9c54-7ba429e13da6})" ) ;
                }
                unsafe impl ::windows::Interface for XmlNodeList {
                    type Vtable = IXmlNodeList_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        2355146103,
                        33700,
                        20161,
                        [156, 84, 123, 164, 41, 225, 61, 166],
                    );
                }
                impl ::windows::RuntimeName for XmlNodeList {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlNodeList";
                }
                impl ::std::convert::From<XmlNodeList> for ::windows::IUnknown {
                    fn from(value: XmlNodeList) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&XmlNodeList> for ::windows::IUnknown {
                    fn from(value: &XmlNodeList) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for XmlNodeList {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &XmlNodeList {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<XmlNodeList> for ::windows::IInspectable {
                    fn from(value: XmlNodeList) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&XmlNodeList> for ::windows::IInspectable {
                    fn from(value: &XmlNodeList) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for XmlNodeList {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a XmlNodeList {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<XmlNodeList>
                    for super::super::super::Foundation::Collections::IIterable<IXmlNode>
                {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlNodeList) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlNodeList>
                    for super::super::super::Foundation::Collections::IIterable<IXmlNode>
                {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlNodeList) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a>
                    ::windows::IntoParam<
                        'a,
                        super::super::super::Foundation::Collections::IIterable<IXmlNode>,
                    > for XmlNodeList
                {
                    fn into_param(
                        self,
                    ) -> ::windows::Param<
                        'a,
                        super::super::super::Foundation::Collections::IIterable<IXmlNode>,
                    > {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a>
                    ::windows::IntoParam<
                        'a,
                        super::super::super::Foundation::Collections::IIterable<IXmlNode>,
                    > for &XmlNodeList
                {
                    fn into_param(
                        self,
                    ) -> ::windows::Param<
                        'a,
                        super::super::super::Foundation::Collections::IIterable<IXmlNode>,
                    > {
                        ::std::convert::TryInto::<
                            super::super::super::Foundation::Collections::IIterable<IXmlNode>,
                        >::try_into(self)
                        .map(::windows::Param::Owned)
                        .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlNodeList>
                    for super::super::super::Foundation::Collections::IVectorView<IXmlNode>
                {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlNodeList) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlNodeList>
                    for super::super::super::Foundation::Collections::IVectorView<IXmlNode>
                {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlNodeList) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a>
                    ::windows::IntoParam<
                        'a,
                        super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                    > for XmlNodeList
                {
                    fn into_param(
                        self,
                    ) -> ::windows::Param<
                        'a,
                        super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                    > {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a>
                    ::windows::IntoParam<
                        'a,
                        super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                    > for &XmlNodeList
                {
                    fn into_param(
                        self,
                    ) -> ::windows::Param<
                        'a,
                        super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                    > {
                        ::std::convert::TryInto::<
                            super::super::super::Foundation::Collections::IVectorView<IXmlNode>,
                        >::try_into(self)
                        .map(::windows::Param::Owned)
                        .unwrap_or(::windows::Param::None)
                    }
                }
                unsafe impl ::std::marker::Send for XmlNodeList {}
                unsafe impl ::std::marker::Sync for XmlNodeList {}
                impl ::std::iter::IntoIterator for XmlNodeList {
                    type Item = IXmlNode;
                    type IntoIter =
                        super::super::super::Foundation::Collections::VectorViewIterator<
                            Self::Item,
                        >;
                    fn into_iter(self) -> Self::IntoIter {
                        ::std::iter::IntoIterator::into_iter(&self)
                    }
                }
                impl ::std::iter::IntoIterator for &XmlNodeList {
                    type Item = IXmlNode;
                    type IntoIter =
                        super::super::super::Foundation::Collections::VectorViewIterator<
                            Self::Item,
                        >;
                    fn into_iter(self) -> Self::IntoIter {
                        super::super::super::Foundation::Collections::VectorViewIterator::new(
                            ::std::convert::TryInto::try_into(self).ok(),
                        )
                    }
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct XmlProcessingInstruction(::windows::IInspectable);
                impl XmlProcessingInstruction {
                    pub fn Target(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn Data(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetData<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = self;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeValue(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SetNodeValue<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeType(&self) -> ::windows::Result<NodeType> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <NodeType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<NodeType>(result__)
                        }
                    }
                    pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn ParentNode(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ChildNodes(&self) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn FirstChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LastChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn PreviousSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NextSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Attributes(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn HasChildNodes(&self) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn OwnerDocument(&self) -> ::windows::Result<XmlDocument> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlDocument as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocument>(result__)
                        }
                    }
                    pub fn InsertBefore<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ReplaceChild<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        childnode: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                childnode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn AppendChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        newchild: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn CloneNode(&self, deep: bool) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).23)(
                                ::std::mem::transmute_copy(this),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NamespaceUri(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).24)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn LocalName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).25)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Prefix(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).26)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Normalize(&self) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).27)(::std::mem::transmute_copy(
                                this,
                            ))
                            .ok()
                        }
                    }
                    pub fn SetPrefix<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).28)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for XmlProcessingInstruction {
                    const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Data.Xml.Dom.XmlProcessingInstruction;{2707fd1e-1e92-4ece-b6f4-26f069078ddc})" ) ;
                }
                unsafe impl ::windows::Interface for XmlProcessingInstruction {
                    type Vtable = IXmlProcessingInstruction_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        654834974,
                        7826,
                        20174,
                        [182, 244, 38, 240, 105, 7, 141, 220],
                    );
                }
                impl ::windows::RuntimeName for XmlProcessingInstruction {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlProcessingInstruction";
                }
                impl ::std::convert::From<XmlProcessingInstruction> for ::windows::IUnknown {
                    fn from(value: XmlProcessingInstruction) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&XmlProcessingInstruction> for ::windows::IUnknown {
                    fn from(value: &XmlProcessingInstruction) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for XmlProcessingInstruction {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &XmlProcessingInstruction {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<XmlProcessingInstruction> for ::windows::IInspectable {
                    fn from(value: XmlProcessingInstruction) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&XmlProcessingInstruction> for ::windows::IInspectable {
                    fn from(value: &XmlProcessingInstruction) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for XmlProcessingInstruction {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a XmlProcessingInstruction {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::TryFrom<XmlProcessingInstruction> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlProcessingInstruction) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlProcessingInstruction> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlProcessingInstruction) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for XmlProcessingInstruction {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for &XmlProcessingInstruction {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::std::convert::TryInto::<IXmlNode>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlProcessingInstruction> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlProcessingInstruction) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlProcessingInstruction> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlProcessingInstruction) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for XmlProcessingInstruction {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for &XmlProcessingInstruction {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlProcessingInstruction> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlProcessingInstruction) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlProcessingInstruction> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlProcessingInstruction) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for XmlProcessingInstruction {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for &XmlProcessingInstruction {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                unsafe impl ::std::marker::Send for XmlProcessingInstruction {}
                unsafe impl ::std::marker::Sync for XmlProcessingInstruction {}
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct XmlText(::windows::IInspectable);
                impl XmlText {
                    pub fn SplitText(&self, offset: u32) -> ::windows::Result<IXmlText> {
                        let this = self;
                        unsafe {
                            let mut result__: <IXmlText as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                &mut result__,
                            )
                            .from_abi::<IXmlText>(result__)
                        }
                    }
                    pub fn Data(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetData<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn Length(&self) -> ::windows::Result<u32> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<u32>(result__)
                        }
                    }
                    pub fn SubstringData(
                        &self,
                        offset: u32,
                        count: u32,
                    ) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                count,
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn AppendData<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        data: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                data.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn InsertData<'a, Param1: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        offset: u32,
                        data: Param1,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                data.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn DeleteData(&self, offset: u32, count: u32) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                count,
                            )
                            .ok()
                        }
                    }
                    pub fn ReplaceData<'a, Param2: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        offset: u32,
                        count: u32,
                        data: Param2,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlCharacterData>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                offset,
                                count,
                                data.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeValue(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn SetNodeValue<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn NodeType(&self) -> ::windows::Result<NodeType> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <NodeType as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<NodeType>(result__)
                        }
                    }
                    pub fn NodeName(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn ParentNode(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).10)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ChildNodes(&self) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).11)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn FirstChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).12)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn LastChild(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).13)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn PreviousSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).14)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NextSibling(&self) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).15)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn Attributes(&self) -> ::windows::Result<XmlNamedNodeMap> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlNamedNodeMap as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).16)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlNamedNodeMap>(result__)
                        }
                    }
                    pub fn HasChildNodes(&self) -> ::windows::Result<bool> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).17)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<bool>(result__)
                        }
                    }
                    pub fn OwnerDocument(&self) -> ::windows::Result<XmlDocument> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <XmlDocument as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).18)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<XmlDocument>(result__)
                        }
                    }
                    pub fn InsertBefore<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).19)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn ReplaceChild<
                        'a,
                        Param0: ::windows::IntoParam<'a, IXmlNode>,
                        Param1: ::windows::IntoParam<'a, IXmlNode>,
                    >(
                        &self,
                        newchild: Param0,
                        referencechild: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).20)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                referencechild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn RemoveChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        childnode: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).21)(
                                ::std::mem::transmute_copy(this),
                                childnode.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn AppendChild<'a, Param0: ::windows::IntoParam<'a, IXmlNode>>(
                        &self,
                        newchild: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).22)(
                                ::std::mem::transmute_copy(this),
                                newchild.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn CloneNode(&self, deep: bool) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).23)(
                                ::std::mem::transmute_copy(this),
                                deep,
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn NamespaceUri(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).24)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn LocalName(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).25)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Prefix(&self) -> ::windows::Result<::windows::IInspectable> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            let mut result__: <::windows::IInspectable as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).26)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::IInspectable>(result__)
                        }
                    }
                    pub fn Normalize(&self) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).27)(::std::mem::transmute_copy(
                                this,
                            ))
                            .ok()
                        }
                    }
                    pub fn SetPrefix<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNode>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).28)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectSingleNode<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodes<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                        &self,
                        xpath: Param0,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn SelectSingleNodeNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<IXmlNode> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <IXmlNode as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<IXmlNode>(result__)
                        }
                    }
                    pub fn SelectNodesNS<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                        Param1: ::windows::IntoParam<'a, ::windows::IInspectable>,
                    >(
                        &self,
                        xpath: Param0,
                        namespaces: Param1,
                    ) -> ::windows::Result<XmlNodeList> {
                        let this = &::windows::Interface::cast::<IXmlNodeSelector>(self)?;
                        unsafe {
                            let mut result__: <XmlNodeList as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).9)(
                                ::std::mem::transmute_copy(this),
                                xpath.into_param().abi(),
                                namespaces.into_param().abi(),
                                &mut result__,
                            )
                            .from_abi::<XmlNodeList>(result__)
                        }
                    }
                    pub fn GetXml(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).6)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn InnerText(&self) -> ::windows::Result<::windows::HSTRING> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                                ::std::mem::zeroed();
                            (::windows::Interface::vtable(this).7)(
                                ::std::mem::transmute_copy(this),
                                &mut result__,
                            )
                            .from_abi::<::windows::HSTRING>(result__)
                        }
                    }
                    pub fn SetInnerText<
                        'a,
                        Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                    >(
                        &self,
                        value: Param0,
                    ) -> ::windows::Result<()> {
                        let this = &::windows::Interface::cast::<IXmlNodeSerializer>(self)?;
                        unsafe {
                            (::windows::Interface::vtable(this).8)(
                                ::std::mem::transmute_copy(this),
                                value.into_param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                unsafe impl ::windows::RuntimeType for XmlText {
                    const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                        b"rc(Windows.Data.Xml.Dom.XmlText;{f931a4cb-308d-4760-a1d5-43b67450ac7e})",
                    );
                }
                unsafe impl ::windows::Interface for XmlText {
                    type Vtable = IXmlText_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        4180780235,
                        12429,
                        18272,
                        [161, 213, 67, 182, 116, 80, 172, 126],
                    );
                }
                impl ::windows::RuntimeName for XmlText {
                    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlText";
                }
                impl ::std::convert::From<XmlText> for ::windows::IUnknown {
                    fn from(value: XmlText) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&XmlText> for ::windows::IUnknown {
                    fn from(value: &XmlText) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for XmlText {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &XmlText {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<XmlText> for ::windows::IInspectable {
                    fn from(value: XmlText) -> Self {
                        value.0
                    }
                }
                impl ::std::convert::From<&XmlText> for ::windows::IInspectable {
                    fn from(value: &XmlText) -> Self {
                        value.0.clone()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for XmlText {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Owned(self.0)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a XmlText {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                        ::windows::Param::Borrowed(&self.0)
                    }
                }
                impl ::std::convert::From<XmlText> for IXmlText {
                    fn from(value: XmlText) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&XmlText> for IXmlText {
                    fn from(value: &XmlText) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlText> for XmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlText> {
                        ::windows::Param::Owned(::std::convert::Into::<IXmlText>::into(self))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlText> for &XmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlText> {
                        ::windows::Param::Owned(::std::convert::Into::<IXmlText>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::TryFrom<XmlText> for IXmlCharacterData {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlText) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlText> for IXmlCharacterData {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlText) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlCharacterData> for XmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlCharacterData> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlCharacterData> for &XmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlCharacterData> {
                        ::std::convert::TryInto::<IXmlCharacterData>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlText> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlText) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlText> for IXmlNode {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlText) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for XmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNode> for &XmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNode> {
                        ::std::convert::TryInto::<IXmlNode>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlText> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlText) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlText> for IXmlNodeSelector {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlText) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for XmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSelector> for &XmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSelector> {
                        ::std::convert::TryInto::<IXmlNodeSelector>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                impl ::std::convert::TryFrom<XmlText> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: XmlText) -> ::windows::Result<Self> {
                        ::std::convert::TryFrom::try_from(&value)
                    }
                }
                impl ::std::convert::TryFrom<&XmlText> for IXmlNodeSerializer {
                    type Error = ::windows::Error;
                    fn try_from(value: &XmlText) -> ::windows::Result<Self> {
                        ::windows::Interface::cast(value)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for XmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::windows::IntoParam::into_param(&self)
                    }
                }
                impl<'a> ::windows::IntoParam<'a, IXmlNodeSerializer> for &XmlText {
                    fn into_param(self) -> ::windows::Param<'a, IXmlNodeSerializer> {
                        ::std::convert::TryInto::<IXmlNodeSerializer>::try_into(self)
                            .map(::windows::Param::Owned)
                            .unwrap_or(::windows::Param::None)
                    }
                }
                unsafe impl ::std::marker::Send for XmlText {}
                unsafe impl ::std::marker::Sync for XmlText {}
            }
        }
    }
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Foundation {
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct AsyncActionCompletedHandler(::windows::IUnknown);
        impl AsyncActionCompletedHandler {
            pub fn new<
                F: FnMut(
                        &::std::option::Option<IAsyncAction>,
                        AsyncStatus,
                    ) -> ::windows::Result<()>
                    + 'static,
            >(
                invoke: F,
            ) -> Self {
                let com = AsyncActionCompletedHandler_box::<F> {
                    vtable: &AsyncActionCompletedHandler_box::<F>::VTABLE,
                    count: ::windows::RefCount::new(1),
                    invoke,
                };
                unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
            }
            pub fn Invoke<'a, Param0: ::windows::IntoParam<'a, IAsyncAction>>(
                &self,
                asyncinfo: Param0,
                asyncstatus: AsyncStatus,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).3)(
                        ::std::mem::transmute_copy(this),
                        asyncinfo.into_param().abi(),
                        asyncstatus,
                    )
                    .ok()
                }
            }
        }
        unsafe impl ::windows::RuntimeType for AsyncActionCompletedHandler {
            const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                b"delegate({a4ed5c81-76c9-40bd-8be6-b1d90fb20ae7})",
            );
        }
        unsafe impl ::windows::Interface for AsyncActionCompletedHandler {
            type Vtable = AsyncActionCompletedHandler_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                2767019137,
                30409,
                16573,
                [139, 230, 177, 217, 15, 178, 10, 231],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct AsyncActionCompletedHandler_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                asyncinfo: ::windows::RawPtr,
                asyncstatus: AsyncStatus,
            ) -> ::windows::HRESULT,
        );
        #[repr(C)]
        struct AsyncActionCompletedHandler_box<
            F: FnMut(&::std::option::Option<IAsyncAction>, AsyncStatus) -> ::windows::Result<()>
                + 'static,
        > {
            vtable: *const AsyncActionCompletedHandler_abi,
            invoke: F,
            count: ::windows::RefCount,
        }
        impl<
                F: FnMut(
                        &::std::option::Option<IAsyncAction>,
                        AsyncStatus,
                    ) -> ::windows::Result<()>
                    + 'static,
            > AsyncActionCompletedHandler_box<F>
        {
            const VTABLE: AsyncActionCompletedHandler_abi = AsyncActionCompletedHandler_abi(
                Self::QueryInterface,
                Self::AddRef,
                Self::Release,
                Self::Invoke,
            );
            unsafe extern "system" fn QueryInterface(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                *interface = if iid == &<AsyncActionCompletedHandler as ::windows::Interface>::IID
                    || iid == &<::windows::IUnknown as ::windows::Interface>::IID
                    || iid == &<::windows::IAgileObject as ::windows::Interface>::IID
                {
                    &mut (*this).vtable as *mut _ as _
                } else {
                    ::std::ptr::null_mut()
                };
                if (*interface).is_null() {
                    ::windows::HRESULT(0x8000_4002)
                } else {
                    (*this).count.add_ref();
                    ::windows::HRESULT(0)
                }
            }
            unsafe extern "system" fn AddRef(this: ::windows::RawPtr) -> u32 {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                (*this).count.add_ref()
            }
            unsafe extern "system" fn Release(this: ::windows::RawPtr) -> u32 {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                let remaining = (*this).count.release();
                if remaining == 0 {
                    Box::from_raw(this);
                }
                remaining
            }
            unsafe extern "system" fn Invoke(
                this: ::windows::RawPtr,
                asyncinfo: ::windows::RawPtr,
                asyncstatus: AsyncStatus,
            ) -> ::windows::HRESULT {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                ((*this).invoke)(
                    &*(&asyncinfo as *const <IAsyncAction as ::windows::Abi>::Abi
                        as *const <IAsyncAction as ::windows::Abi>::DefaultType),
                    asyncstatus,
                )
                .into()
            }
        }
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct AsyncOperationCompletedHandler<TResult>(
            ::windows::IUnknown,
            ::std::marker::PhantomData<TResult>,
        )
        where
            TResult: ::windows::RuntimeType + 'static;
        impl<TResult: ::windows::RuntimeType + 'static> AsyncOperationCompletedHandler<TResult> {
            pub fn new<
                F: FnMut(
                        &::std::option::Option<IAsyncOperation<TResult>>,
                        AsyncStatus,
                    ) -> ::windows::Result<()>
                    + 'static,
            >(
                invoke: F,
            ) -> Self {
                let com = AsyncOperationCompletedHandler_box::<TResult, F> {
                    vtable: &AsyncOperationCompletedHandler_box::<TResult, F>::VTABLE,
                    count: ::windows::RefCount::new(1),
                    invoke,
                };
                unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
            }
            pub fn Invoke<'a, Param0: ::windows::IntoParam<'a, IAsyncOperation<TResult>>>(
                &self,
                asyncinfo: Param0,
                asyncstatus: AsyncStatus,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).3)(
                        ::std::mem::transmute_copy(this),
                        asyncinfo.into_param().abi(),
                        asyncstatus,
                    )
                    .ok()
                }
            }
        }
        unsafe impl<TResult: ::windows::RuntimeType + 'static> ::windows::RuntimeType
            for AsyncOperationCompletedHandler<TResult>
        {
            const SIGNATURE: ::windows::ConstBuffer = {
                ::windows::ConstBuffer::new()
                    .push_slice(b"pinterface(")
                    .push_slice(b"{fcdcf02c-e5d8-4478-915a-4d90b74b83a5}")
                    .push_slice(b";")
                    .push_other(<TResult as ::windows::RuntimeType>::SIGNATURE)
                    .push_slice(b")")
            };
        }
        unsafe impl<TResult: ::windows::RuntimeType + 'static> ::windows::Interface
            for AsyncOperationCompletedHandler<TResult>
        {
            type Vtable = AsyncOperationCompletedHandler_abi<TResult>;
            const IID: ::windows::Guid = ::windows::Guid::from_signature(
                <AsyncOperationCompletedHandler<TResult> as ::windows::RuntimeType>::SIGNATURE,
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct AsyncOperationCompletedHandler_abi<TResult>(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                asyncinfo: ::windows::RawPtr,
                asyncstatus: AsyncStatus,
            ) -> ::windows::HRESULT,
            pub ::std::marker::PhantomData<TResult>,
        )
        where
            TResult: ::windows::RuntimeType + 'static;
        #[repr(C)]
        struct AsyncOperationCompletedHandler_box<
            TResult,
            F: FnMut(
                    &::std::option::Option<IAsyncOperation<TResult>>,
                    AsyncStatus,
                ) -> ::windows::Result<()>
                + 'static,
        >
        where
            TResult: ::windows::RuntimeType + 'static,
        {
            vtable: *const AsyncOperationCompletedHandler_abi<TResult>,
            invoke: F,
            count: ::windows::RefCount,
        }
        impl<
                TResult: ::windows::RuntimeType + 'static,
                F: FnMut(
                        &::std::option::Option<IAsyncOperation<TResult>>,
                        AsyncStatus,
                    ) -> ::windows::Result<()>
                    + 'static,
            > AsyncOperationCompletedHandler_box<TResult, F>
        {
            const VTABLE: AsyncOperationCompletedHandler_abi<TResult> =
                AsyncOperationCompletedHandler_abi::<TResult>(
                    Self::QueryInterface,
                    Self::AddRef,
                    Self::Release,
                    Self::Invoke,
                    ::std::marker::PhantomData::<TResult>,
                );
            unsafe extern "system" fn QueryInterface(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                *interface = if iid
                    == &<AsyncOperationCompletedHandler<TResult> as ::windows::Interface>::IID
                    || iid == &<::windows::IUnknown as ::windows::Interface>::IID
                    || iid == &<::windows::IAgileObject as ::windows::Interface>::IID
                {
                    &mut (*this).vtable as *mut _ as _
                } else {
                    ::std::ptr::null_mut()
                };
                if (*interface).is_null() {
                    ::windows::HRESULT(0x8000_4002)
                } else {
                    (*this).count.add_ref();
                    ::windows::HRESULT(0)
                }
            }
            unsafe extern "system" fn AddRef(this: ::windows::RawPtr) -> u32 {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                (*this).count.add_ref()
            }
            unsafe extern "system" fn Release(this: ::windows::RawPtr) -> u32 {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                let remaining = (*this).count.release();
                if remaining == 0 {
                    Box::from_raw(this);
                }
                remaining
            }
            unsafe extern "system" fn Invoke(
                this: ::windows::RawPtr,
                asyncinfo: ::windows::RawPtr,
                asyncstatus: AsyncStatus,
            ) -> ::windows::HRESULT {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                ((*this).invoke)(
                    &*(&asyncinfo as *const <IAsyncOperation<TResult> as ::windows::Abi>::Abi
                        as *const <IAsyncOperation<TResult> as ::windows::Abi>::DefaultType),
                    asyncstatus,
                )
                .into()
            }
        }
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: marker :: Copy,
            :: std :: clone :: Clone,
            :: std :: default :: Default,
            :: std :: fmt :: Debug,
        )]
        #[repr(transparent)]
        pub struct AsyncStatus(pub i32);
        impl AsyncStatus {
            pub const Canceled: AsyncStatus = AsyncStatus(2i32);
            pub const Completed: AsyncStatus = AsyncStatus(1i32);
            pub const Error: AsyncStatus = AsyncStatus(3i32);
            pub const Started: AsyncStatus = AsyncStatus(0i32);
        }
        impl ::std::convert::From<i32> for AsyncStatus {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }
        unsafe impl ::windows::Abi for AsyncStatus {
            type Abi = Self;
            type DefaultType = Self;
        }
        unsafe impl ::windows::RuntimeType for AsyncStatus {
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"enum(Windows.Foundation.AsyncStatus;i4)");
        }
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IAsyncAction(::windows::IInspectable);
        unsafe impl ::windows::Interface for IAsyncAction {
            type Vtable = IAsyncAction_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                1516535814,
                33850,
                19881,
                [134, 91, 157, 38, 229, 223, 173, 123],
            );
        }
        impl IAsyncAction {
            pub fn SetCompleted<
                'a,
                Param0: ::windows::IntoParam<'a, AsyncActionCompletedHandler>,
            >(
                &self,
                handler: Param0,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        handler.into_param().abi(),
                    )
                    .ok()
                }
            }
            pub fn Completed(&self) -> ::windows::Result<AsyncActionCompletedHandler> {
                let this = self;
                unsafe {
                    let mut result__: <AsyncActionCompletedHandler as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<AsyncActionCompletedHandler>(result__)
                }
            }
            pub fn GetResults(&self) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok()
                }
            }
            pub fn Id(&self) -> ::windows::Result<u32> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self)?;
                unsafe {
                    let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<u32>(result__)
                }
            }
            pub fn Status(&self) -> ::windows::Result<AsyncStatus> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self)?;
                unsafe {
                    let mut result__: <AsyncStatus as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<AsyncStatus>(result__)
                }
            }
            pub fn ErrorCode(&self) -> ::windows::Result<::windows::HRESULT> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self)?;
                unsafe {
                    let mut result__: <::windows::HRESULT as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).8)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HRESULT>(result__)
                }
            }
            pub fn Cancel(&self) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self)?;
                unsafe {
                    (::windows::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok()
                }
            }
            pub fn Close(&self) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self)?;
                unsafe {
                    (::windows::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok()
                }
            }
            pub fn get(&self) -> ::windows::Result<()> {
                if self.Status()? == AsyncStatus::Started {
                    let (waiter, signaler) = ::windows::Waiter::new();
                    self.SetCompleted(AsyncActionCompletedHandler::new(move |_sender, _args| {
                        unsafe {
                            signaler.signal();
                        }
                        Ok(())
                    }))?;
                }
                self.GetResults()
            }
        }
        unsafe impl ::windows::RuntimeType for IAsyncAction {
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{5a648006-843a-4da9-865b-9d26e5dfad7b}");
        }
        impl ::std::future::Future for IAsyncAction {
            type Output = ::windows::Result<()>;
            fn poll(
                self: ::std::pin::Pin<&mut Self>,
                context: &mut ::std::task::Context,
            ) -> ::std::task::Poll<Self::Output> {
                if self.Status()? == AsyncStatus::Started {
                    let waker = context.waker().clone();
                    let _ = self.SetCompleted(AsyncActionCompletedHandler::new(
                        move |_sender, _args| {
                            waker.wake_by_ref();
                            Ok(())
                        },
                    ));
                    ::std::task::Poll::Pending
                } else {
                    ::std::task::Poll::Ready(self.GetResults())
                }
            }
        }
        impl ::std::convert::From<IAsyncAction> for ::windows::IUnknown {
            fn from(value: IAsyncAction) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl ::std::convert::From<&IAsyncAction> for ::windows::IUnknown {
            fn from(value: &IAsyncAction) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IAsyncAction {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IAsyncAction {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<IAsyncAction> for ::windows::IInspectable {
            fn from(value: IAsyncAction) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IAsyncAction> for ::windows::IInspectable {
            fn from(value: &IAsyncAction) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for IAsyncAction {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IAsyncAction {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        impl ::std::convert::TryFrom<IAsyncAction> for IAsyncInfo {
            type Error = ::windows::Error;
            fn try_from(value: IAsyncAction) -> ::windows::Result<Self> {
                ::std::convert::TryFrom::try_from(&value)
            }
        }
        impl ::std::convert::TryFrom<&IAsyncAction> for IAsyncInfo {
            type Error = ::windows::Error;
            fn try_from(value: &IAsyncAction) -> ::windows::Result<Self> {
                ::windows::Interface::cast(value)
            }
        }
        impl<'a> ::windows::IntoParam<'a, IAsyncInfo> for IAsyncAction {
            fn into_param(self) -> ::windows::Param<'a, IAsyncInfo> {
                ::windows::IntoParam::into_param(&self)
            }
        }
        impl<'a> ::windows::IntoParam<'a, IAsyncInfo> for &IAsyncAction {
            fn into_param(self) -> ::windows::Param<'a, IAsyncInfo> {
                ::std::convert::TryInto::<IAsyncInfo>::try_into(self)
                    .map(::windows::Param::Owned)
                    .unwrap_or(::windows::Param::None)
            }
        }
        unsafe impl ::std::marker::Send for IAsyncAction {}
        unsafe impl ::std::marker::Sync for IAsyncAction {}
        #[repr(C)]
        #[doc(hidden)]
        pub struct IAsyncAction_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                handler: ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IAsyncInfo(::windows::IInspectable);
        unsafe impl ::windows::Interface for IAsyncInfo {
            type Vtable = IAsyncInfo_abi;
            const IID: ::windows::Guid =
                ::windows::Guid::from_values(54, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
        }
        impl IAsyncInfo {
            pub fn Id(&self) -> ::windows::Result<u32> {
                let this = self;
                unsafe {
                    let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<u32>(result__)
                }
            }
            pub fn Status(&self) -> ::windows::Result<AsyncStatus> {
                let this = self;
                unsafe {
                    let mut result__: <AsyncStatus as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<AsyncStatus>(result__)
                }
            }
            pub fn ErrorCode(&self) -> ::windows::Result<::windows::HRESULT> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HRESULT as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).8)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HRESULT>(result__)
                }
            }
            pub fn Cancel(&self) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok()
                }
            }
            pub fn Close(&self) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok()
                }
            }
        }
        unsafe impl ::windows::RuntimeType for IAsyncInfo {
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{00000036-0000-0000-c000-000000000046}");
        }
        impl ::std::convert::From<IAsyncInfo> for ::windows::IUnknown {
            fn from(value: IAsyncInfo) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl ::std::convert::From<&IAsyncInfo> for ::windows::IUnknown {
            fn from(value: &IAsyncInfo) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IAsyncInfo {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IAsyncInfo {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<IAsyncInfo> for ::windows::IInspectable {
            fn from(value: IAsyncInfo) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IAsyncInfo> for ::windows::IInspectable {
            fn from(value: &IAsyncInfo) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for IAsyncInfo {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IAsyncInfo {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IAsyncInfo_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut u32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut AsyncStatus,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::HRESULT,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IAsyncOperation<TResult>(
            ::windows::IInspectable,
            ::std::marker::PhantomData<TResult>,
        )
        where
            TResult: ::windows::RuntimeType + 'static;
        unsafe impl<TResult: ::windows::RuntimeType + 'static> ::windows::Interface
            for IAsyncOperation<TResult>
        {
            type Vtable = IAsyncOperation_abi<TResult>;
            const IID: ::windows::Guid = ::windows::Guid::from_signature(
                <IAsyncOperation<TResult> as ::windows::RuntimeType>::SIGNATURE,
            );
        }
        impl<TResult: ::windows::RuntimeType + 'static> IAsyncOperation<TResult> {
            pub fn SetCompleted<
                'a,
                Param0: ::windows::IntoParam<'a, AsyncOperationCompletedHandler<TResult>>,
            >(
                &self,
                handler: Param0,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        handler.into_param().abi(),
                    )
                    .ok()
                }
            }
            pub fn Completed(&self) -> ::windows::Result<AsyncOperationCompletedHandler<TResult>> {
                let this = self;
                unsafe {
                    let mut result__ : < AsyncOperationCompletedHandler < TResult > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).7)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<AsyncOperationCompletedHandler<TResult>>(result__)
                }
            }
            pub fn GetResults(&self) -> ::windows::Result<TResult> {
                let this = self;
                unsafe {
                    let mut result__: <TResult as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).8)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<TResult>(result__)
                }
            }
            pub fn Id(&self) -> ::windows::Result<u32> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self)?;
                unsafe {
                    let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<u32>(result__)
                }
            }
            pub fn Status(&self) -> ::windows::Result<AsyncStatus> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self)?;
                unsafe {
                    let mut result__: <AsyncStatus as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<AsyncStatus>(result__)
                }
            }
            pub fn ErrorCode(&self) -> ::windows::Result<::windows::HRESULT> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self)?;
                unsafe {
                    let mut result__: <::windows::HRESULT as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).8)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HRESULT>(result__)
                }
            }
            pub fn Cancel(&self) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self)?;
                unsafe {
                    (::windows::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok()
                }
            }
            pub fn Close(&self) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self)?;
                unsafe {
                    (::windows::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok()
                }
            }
            pub fn get(&self) -> ::windows::Result<TResult> {
                if self.Status()? == AsyncStatus::Started {
                    let (waiter, signaler) = ::windows::Waiter::new();
                    self.SetCompleted(AsyncOperationCompletedHandler::new(
                        move |_sender, _args| {
                            unsafe {
                                signaler.signal();
                            }
                            Ok(())
                        },
                    ))?;
                }
                self.GetResults()
            }
        }
        unsafe impl<TResult: ::windows::RuntimeType + 'static> ::windows::RuntimeType
            for IAsyncOperation<TResult>
        {
            const SIGNATURE: ::windows::ConstBuffer = {
                ::windows::ConstBuffer::new()
                    .push_slice(b"pinterface(")
                    .push_slice(b"{9fc2b0bb-e446-44e2-aa61-9cab8f636af2}")
                    .push_slice(b";")
                    .push_other(<TResult as ::windows::RuntimeType>::SIGNATURE)
                    .push_slice(b")")
            };
        }
        impl<TResult: ::windows::RuntimeType + 'static> ::std::future::Future for IAsyncOperation<TResult> {
            type Output = ::windows::Result<TResult>;
            fn poll(
                self: ::std::pin::Pin<&mut Self>,
                context: &mut ::std::task::Context,
            ) -> ::std::task::Poll<Self::Output> {
                if self.Status()? == AsyncStatus::Started {
                    let waker = context.waker().clone();
                    let _ = self.SetCompleted(AsyncOperationCompletedHandler::new(
                        move |_sender, _args| {
                            waker.wake_by_ref();
                            Ok(())
                        },
                    ));
                    ::std::task::Poll::Pending
                } else {
                    ::std::task::Poll::Ready(self.GetResults())
                }
            }
        }
        impl<TResult: ::windows::RuntimeType + 'static>
            ::std::convert::From<IAsyncOperation<TResult>> for ::windows::IUnknown
        {
            fn from(value: IAsyncOperation<TResult>) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl<TResult: ::windows::RuntimeType + 'static>
            ::std::convert::From<&IAsyncOperation<TResult>> for ::windows::IUnknown
        {
            fn from(value: &IAsyncOperation<TResult>) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }
        impl<'a, TResult: ::windows::RuntimeType + 'static>
            ::windows::IntoParam<'a, ::windows::IUnknown> for IAsyncOperation<TResult>
        {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
            }
        }
        impl<'a, TResult: ::windows::RuntimeType + 'static>
            ::windows::IntoParam<'a, ::windows::IUnknown> for &IAsyncOperation<TResult>
        {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl<TResult: ::windows::RuntimeType + 'static>
            ::std::convert::From<IAsyncOperation<TResult>> for ::windows::IInspectable
        {
            fn from(value: IAsyncOperation<TResult>) -> Self {
                value.0
            }
        }
        impl<TResult: ::windows::RuntimeType + 'static>
            ::std::convert::From<&IAsyncOperation<TResult>> for ::windows::IInspectable
        {
            fn from(value: &IAsyncOperation<TResult>) -> Self {
                value.0.clone()
            }
        }
        impl<'a, TResult: ::windows::RuntimeType + 'static>
            ::windows::IntoParam<'a, ::windows::IInspectable> for IAsyncOperation<TResult>
        {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a, TResult: ::windows::RuntimeType + 'static>
            ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IAsyncOperation<TResult>
        {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        impl<TResult: ::windows::RuntimeType + 'static>
            ::std::convert::TryFrom<IAsyncOperation<TResult>> for IAsyncInfo
        {
            type Error = ::windows::Error;
            fn try_from(value: IAsyncOperation<TResult>) -> ::windows::Result<Self> {
                ::std::convert::TryFrom::try_from(&value)
            }
        }
        impl<TResult: ::windows::RuntimeType + 'static>
            ::std::convert::TryFrom<&IAsyncOperation<TResult>> for IAsyncInfo
        {
            type Error = ::windows::Error;
            fn try_from(value: &IAsyncOperation<TResult>) -> ::windows::Result<Self> {
                ::windows::Interface::cast(value)
            }
        }
        impl<'a, TResult: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IAsyncInfo>
            for IAsyncOperation<TResult>
        {
            fn into_param(self) -> ::windows::Param<'a, IAsyncInfo> {
                ::windows::IntoParam::into_param(&self)
            }
        }
        impl<'a, TResult: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IAsyncInfo>
            for &IAsyncOperation<TResult>
        {
            fn into_param(self) -> ::windows::Param<'a, IAsyncInfo> {
                ::std::convert::TryInto::<IAsyncInfo>::try_into(self)
                    .map(::windows::Param::Owned)
                    .unwrap_or(::windows::Param::None)
            }
        }
        unsafe impl<TResult: ::windows::RuntimeType + 'static> ::std::marker::Send
            for IAsyncOperation<TResult>
        {
        }
        unsafe impl<TResult: ::windows::RuntimeType + 'static> ::std::marker::Sync
            for IAsyncOperation<TResult>
        {
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IAsyncOperation_abi<TResult>(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                handler: ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut <TResult as ::windows::Abi>::Abi,
            ) -> ::windows::HRESULT,
            pub ::std::marker::PhantomData<TResult>,
        )
        where
            TResult: ::windows::RuntimeType + 'static;
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IStringable(::windows::IInspectable);
        unsafe impl ::windows::Interface for IStringable {
            type Vtable = IStringable_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                2520162132,
                36534,
                18672,
                [171, 206, 193, 178, 17, 230, 39, 195],
            );
        }
        impl IStringable {
            pub fn ToString(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
        }
        unsafe impl ::windows::RuntimeType for IStringable {
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{96369f54-8eb6-48f0-abce-c1b211e627c3}");
        }
        impl ::std::convert::From<IStringable> for ::windows::IUnknown {
            fn from(value: IStringable) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl ::std::convert::From<&IStringable> for ::windows::IUnknown {
            fn from(value: &IStringable) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IStringable {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IStringable {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<IStringable> for ::windows::IInspectable {
            fn from(value: IStringable) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IStringable> for ::windows::IInspectable {
            fn from(value: &IStringable) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for IStringable {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IStringable {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStringable_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IUriEscapeStatics(::windows::IInspectable);
        unsafe impl ::windows::Interface for IUriEscapeStatics {
            type Vtable = IUriEscapeStatics_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                3251909306,
                51236,
                17490,
                [167, 253, 81, 43, 195, 187, 233, 161],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IUriEscapeStatics_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                tounescape: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                toescape: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IUriRuntimeClass(::windows::IInspectable);
        unsafe impl ::windows::Interface for IUriRuntimeClass {
            type Vtable = IUriRuntimeClass_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                2654363223,
                18610,
                16736,
                [149, 111, 199, 56, 81, 32, 187, 252],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IUriRuntimeClass_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut bool,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                puri: ::windows::RawPtr,
                result__: *mut bool,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                relativeuri: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IUriRuntimeClassFactory(::windows::IInspectable);
        unsafe impl ::windows::Interface for IUriRuntimeClassFactory {
            type Vtable = IUriRuntimeClassFactory_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                1151957359,
                29246,
                20447,
                [162, 24, 3, 62, 117, 176, 192, 132],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IUriRuntimeClassFactory_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                uri: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                baseuri: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                relativeuri: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IUriRuntimeClassWithAbsoluteCanonicalUri(::windows::IInspectable);
        unsafe impl ::windows::Interface for IUriRuntimeClassWithAbsoluteCanonicalUri {
            type Vtable = IUriRuntimeClassWithAbsoluteCanonicalUri_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                1972213345,
                8732,
                18447,
                [163, 57, 80, 101, 102, 115, 244, 111],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IUriRuntimeClassWithAbsoluteCanonicalUri_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IWwwFormUrlDecoderEntry(::windows::IInspectable);
        unsafe impl ::windows::Interface for IWwwFormUrlDecoderEntry {
            type Vtable = IWwwFormUrlDecoderEntry_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                308180017,
                63096,
                20110,
                [182, 112, 32, 169, 176, 108, 81, 45],
            );
        }
        impl IWwwFormUrlDecoderEntry {
            pub fn Name(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Value(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
        }
        unsafe impl ::windows::RuntimeType for IWwwFormUrlDecoderEntry {
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{125e7431-f678-4e8e-b670-20a9b06c512d}");
        }
        impl ::std::convert::From<IWwwFormUrlDecoderEntry> for ::windows::IUnknown {
            fn from(value: IWwwFormUrlDecoderEntry) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl ::std::convert::From<&IWwwFormUrlDecoderEntry> for ::windows::IUnknown {
            fn from(value: &IWwwFormUrlDecoderEntry) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for IWwwFormUrlDecoderEntry {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &IWwwFormUrlDecoderEntry {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<IWwwFormUrlDecoderEntry> for ::windows::IInspectable {
            fn from(value: IWwwFormUrlDecoderEntry) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IWwwFormUrlDecoderEntry> for ::windows::IInspectable {
            fn from(value: &IWwwFormUrlDecoderEntry) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for IWwwFormUrlDecoderEntry {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IWwwFormUrlDecoderEntry {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IWwwFormUrlDecoderEntry_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IWwwFormUrlDecoderRuntimeClass(::windows::IInspectable);
        unsafe impl ::windows::Interface for IWwwFormUrlDecoderRuntimeClass {
            type Vtable = IWwwFormUrlDecoderRuntimeClass_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                3562669137,
                61989,
                17730,
                [146, 150, 14, 29, 245, 210, 84, 223],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IWwwFormUrlDecoderRuntimeClass_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                name: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                result__: *mut ::std::mem::ManuallyDrop<::windows::HSTRING>,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IWwwFormUrlDecoderRuntimeClassFactory(::windows::IInspectable);
        unsafe impl ::windows::Interface for IWwwFormUrlDecoderRuntimeClassFactory {
            type Vtable = IWwwFormUrlDecoderRuntimeClassFactory_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                1535929149,
                9390,
                16821,
                [161, 191, 240, 195, 213, 68, 132, 91],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IWwwFormUrlDecoderRuntimeClassFactory_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                query: ::std::mem::ManuallyDrop<::windows::HSTRING>,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct Uri(::windows::IInspectable);
        impl Uri {
            pub fn AbsoluteUri(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn DisplayUri(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Domain(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).8)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Extension(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).9)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Fragment(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).10)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Host(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Password(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).12)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Path(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).13)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Query(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).14)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn QueryParsed(&self) -> ::windows::Result<WwwFormUrlDecoder> {
                let this = self;
                unsafe {
                    let mut result__: <WwwFormUrlDecoder as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).15)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<WwwFormUrlDecoder>(result__)
                }
            }
            pub fn RawUri(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).16)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn SchemeName(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).17)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn UserName(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).18)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn Port(&self) -> ::windows::Result<i32> {
                let this = self;
                unsafe {
                    let mut result__: <i32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).19)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<i32>(result__)
                }
            }
            pub fn Suspicious(&self) -> ::windows::Result<bool> {
                let this = self;
                unsafe {
                    let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).20)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<bool>(result__)
                }
            }
            pub fn Equals<'a, Param0: ::windows::IntoParam<'a, Uri>>(
                &self,
                puri: Param0,
            ) -> ::windows::Result<bool> {
                let this = self;
                unsafe {
                    let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).21)(
                        ::std::mem::transmute_copy(this),
                        puri.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<bool>(result__)
                }
            }
            pub fn CombineUri<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                &self,
                relativeuri: Param0,
            ) -> ::windows::Result<Uri> {
                let this = self;
                unsafe {
                    let mut result__: <Uri as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).22)(
                        ::std::mem::transmute_copy(this),
                        relativeuri.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<Uri>(result__)
                }
            }
            pub fn ToString(&self) -> ::windows::Result<::windows::HSTRING> {
                let this = &::windows::Interface::cast::<IStringable>(self)?;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn AbsoluteCanonicalUri(&self) -> ::windows::Result<::windows::HSTRING> {
                let this =
                    &::windows::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn DisplayIri(&self) -> ::windows::Result<::windows::HSTRING> {
                let this =
                    &::windows::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn UnescapeComponent<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                tounescape: Param0,
            ) -> ::windows::Result<::windows::HSTRING> {
                Self::IUriEscapeStatics(|this| unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        tounescape.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                })
            }
            pub fn EscapeComponent<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                toescape: Param0,
            ) -> ::windows::Result<::windows::HSTRING> {
                Self::IUriEscapeStatics(|this| unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::std::mem::transmute_copy(this),
                        toescape.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                })
            }
            pub fn CreateUri<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                uri: Param0,
            ) -> ::windows::Result<Uri> {
                Self::IUriRuntimeClassFactory(|this| unsafe {
                    let mut result__: <Uri as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        uri.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<Uri>(result__)
                })
            }
            pub fn CreateWithRelativeUri<
                'a,
                Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
                Param1: ::windows::IntoParam<'a, ::windows::HSTRING>,
            >(
                baseuri: Param0,
                relativeuri: Param1,
            ) -> ::windows::Result<Uri> {
                Self::IUriRuntimeClassFactory(|this| unsafe {
                    let mut result__: <Uri as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::std::mem::transmute_copy(this),
                        baseuri.into_param().abi(),
                        relativeuri.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<Uri>(result__)
                })
            }
            pub fn IUriEscapeStatics<R, F: FnOnce(&IUriEscapeStatics) -> ::windows::Result<R>>(
                callback: F,
            ) -> ::windows::Result<R> {
                static mut SHARED: ::windows::FactoryCache<Uri, IUriEscapeStatics> =
                    ::windows::FactoryCache::new();
                unsafe { SHARED.call(callback) }
            }
            pub fn IUriRuntimeClassFactory<
                R,
                F: FnOnce(&IUriRuntimeClassFactory) -> ::windows::Result<R>,
            >(
                callback: F,
            ) -> ::windows::Result<R> {
                static mut SHARED: ::windows::FactoryCache<Uri, IUriRuntimeClassFactory> =
                    ::windows::FactoryCache::new();
                unsafe { SHARED.call(callback) }
            }
        }
        unsafe impl ::windows::RuntimeType for Uri {
            const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                b"rc(Windows.Foundation.Uri;{9e365e57-48b2-4160-956f-c7385120bbfc})",
            );
        }
        unsafe impl ::windows::Interface for Uri {
            type Vtable = IUriRuntimeClass_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                2654363223,
                18610,
                16736,
                [149, 111, 199, 56, 81, 32, 187, 252],
            );
        }
        impl ::windows::RuntimeName for Uri {
            const NAME: &'static str = "Windows.Foundation.Uri";
        }
        impl ::std::convert::From<Uri> for ::windows::IUnknown {
            fn from(value: Uri) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl ::std::convert::From<&Uri> for ::windows::IUnknown {
            fn from(value: &Uri) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for Uri {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &Uri {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<Uri> for ::windows::IInspectable {
            fn from(value: Uri) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&Uri> for ::windows::IInspectable {
            fn from(value: &Uri) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for Uri {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a Uri {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        impl ::std::convert::TryFrom<Uri> for IStringable {
            type Error = ::windows::Error;
            fn try_from(value: Uri) -> ::windows::Result<Self> {
                ::std::convert::TryFrom::try_from(&value)
            }
        }
        impl ::std::convert::TryFrom<&Uri> for IStringable {
            type Error = ::windows::Error;
            fn try_from(value: &Uri) -> ::windows::Result<Self> {
                ::windows::Interface::cast(value)
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStringable> for Uri {
            fn into_param(self) -> ::windows::Param<'a, IStringable> {
                ::windows::IntoParam::into_param(&self)
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStringable> for &Uri {
            fn into_param(self) -> ::windows::Param<'a, IStringable> {
                ::std::convert::TryInto::<IStringable>::try_into(self)
                    .map(::windows::Param::Owned)
                    .unwrap_or(::windows::Param::None)
            }
        }
        unsafe impl ::std::marker::Send for Uri {}
        unsafe impl ::std::marker::Sync for Uri {}
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct WwwFormUrlDecoder(::windows::IInspectable);
        impl WwwFormUrlDecoder {
            pub fn GetFirstValueByName<'a, Param0: ::windows::IntoParam<'a, ::windows::HSTRING>>(
                &self,
                name: Param0,
            ) -> ::windows::Result<::windows::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HSTRING as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        name.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<::windows::HSTRING>(result__)
                }
            }
            pub fn First(
                &self,
            ) -> ::windows::Result<Collections::IIterator<IWwwFormUrlDecoderEntry>> {
                let this = &::windows::Interface::cast::<
                    Collections::IIterable<IWwwFormUrlDecoderEntry>,
                >(self)?;
                unsafe {
                    let mut result__ : < Collections:: IIterator :: < IWwwFormUrlDecoderEntry > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<Collections::IIterator<IWwwFormUrlDecoderEntry>>(result__)
                }
            }
            pub fn GetAt(&self, index: u32) -> ::windows::Result<IWwwFormUrlDecoderEntry> {
                let this = &::windows::Interface::cast::<
                    Collections::IVectorView<IWwwFormUrlDecoderEntry>,
                >(self)?;
                unsafe {
                    let mut result__: <IWwwFormUrlDecoderEntry as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        index,
                        &mut result__,
                    )
                    .from_abi::<IWwwFormUrlDecoderEntry>(result__)
                }
            }
            pub fn Size(&self) -> ::windows::Result<u32> {
                let this = &::windows::Interface::cast::<
                    Collections::IVectorView<IWwwFormUrlDecoderEntry>,
                >(self)?;
                unsafe {
                    let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::std::mem::transmute_copy(this),
                        &mut result__,
                    )
                    .from_abi::<u32>(result__)
                }
            }
            pub fn IndexOf<'a, Param0: ::windows::IntoParam<'a, IWwwFormUrlDecoderEntry>>(
                &self,
                value: Param0,
                index: &mut u32,
            ) -> ::windows::Result<bool> {
                let this = &::windows::Interface::cast::<
                    Collections::IVectorView<IWwwFormUrlDecoderEntry>,
                >(self)?;
                unsafe {
                    let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).8)(
                        ::std::mem::transmute_copy(this),
                        value.into_param().abi(),
                        index,
                        &mut result__,
                    )
                    .from_abi::<bool>(result__)
                }
            }
            pub fn GetMany(
                &self,
                startindex: u32,
                items: &mut [<IWwwFormUrlDecoderEntry as ::windows::Abi>::DefaultType],
            ) -> ::windows::Result<u32> {
                let this = &::windows::Interface::cast::<
                    Collections::IVectorView<IWwwFormUrlDecoderEntry>,
                >(self)?;
                unsafe {
                    let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).9)(
                        ::std::mem::transmute_copy(this),
                        startindex,
                        items.len() as u32,
                        ::std::mem::transmute_copy(&items),
                        &mut result__,
                    )
                    .from_abi::<u32>(result__)
                }
            }
            pub fn CreateWwwFormUrlDecoder<
                'a,
                Param0: ::windows::IntoParam<'a, ::windows::HSTRING>,
            >(
                query: Param0,
            ) -> ::windows::Result<WwwFormUrlDecoder> {
                Self::IWwwFormUrlDecoderRuntimeClassFactory(|this| unsafe {
                    let mut result__: <WwwFormUrlDecoder as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(
                        ::std::mem::transmute_copy(this),
                        query.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<WwwFormUrlDecoder>(result__)
                })
            }
            pub fn IWwwFormUrlDecoderRuntimeClassFactory<
                R,
                F: FnOnce(&IWwwFormUrlDecoderRuntimeClassFactory) -> ::windows::Result<R>,
            >(
                callback: F,
            ) -> ::windows::Result<R> {
                static mut SHARED: ::windows::FactoryCache<
                    WwwFormUrlDecoder,
                    IWwwFormUrlDecoderRuntimeClassFactory,
                > = ::windows::FactoryCache::new();
                unsafe { SHARED.call(callback) }
            }
        }
        unsafe impl ::windows::RuntimeType for WwwFormUrlDecoder {
            const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                b"rc(Windows.Foundation.WwwFormUrlDecoder;{d45a0451-f225-4542-9296-0e1df5d254df})",
            );
        }
        unsafe impl ::windows::Interface for WwwFormUrlDecoder {
            type Vtable = IWwwFormUrlDecoderRuntimeClass_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                3562669137,
                61989,
                17730,
                [146, 150, 14, 29, 245, 210, 84, 223],
            );
        }
        impl ::windows::RuntimeName for WwwFormUrlDecoder {
            const NAME: &'static str = "Windows.Foundation.WwwFormUrlDecoder";
        }
        impl ::std::convert::From<WwwFormUrlDecoder> for ::windows::IUnknown {
            fn from(value: WwwFormUrlDecoder) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl ::std::convert::From<&WwwFormUrlDecoder> for ::windows::IUnknown {
            fn from(value: &WwwFormUrlDecoder) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for WwwFormUrlDecoder {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &WwwFormUrlDecoder {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<WwwFormUrlDecoder> for ::windows::IInspectable {
            fn from(value: WwwFormUrlDecoder) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&WwwFormUrlDecoder> for ::windows::IInspectable {
            fn from(value: &WwwFormUrlDecoder) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for WwwFormUrlDecoder {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::IInspectable> for &'a WwwFormUrlDecoder {
            fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        impl ::std::convert::TryFrom<WwwFormUrlDecoder>
            for Collections::IIterable<IWwwFormUrlDecoderEntry>
        {
            type Error = ::windows::Error;
            fn try_from(value: WwwFormUrlDecoder) -> ::windows::Result<Self> {
                ::std::convert::TryFrom::try_from(&value)
            }
        }
        impl ::std::convert::TryFrom<&WwwFormUrlDecoder>
            for Collections::IIterable<IWwwFormUrlDecoderEntry>
        {
            type Error = ::windows::Error;
            fn try_from(value: &WwwFormUrlDecoder) -> ::windows::Result<Self> {
                ::windows::Interface::cast(value)
            }
        }
        impl<'a> ::windows::IntoParam<'a, Collections::IIterable<IWwwFormUrlDecoderEntry>>
            for WwwFormUrlDecoder
        {
            fn into_param(
                self,
            ) -> ::windows::Param<'a, Collections::IIterable<IWwwFormUrlDecoderEntry>> {
                ::windows::IntoParam::into_param(&self)
            }
        }
        impl<'a> ::windows::IntoParam<'a, Collections::IIterable<IWwwFormUrlDecoderEntry>>
            for &WwwFormUrlDecoder
        {
            fn into_param(
                self,
            ) -> ::windows::Param<'a, Collections::IIterable<IWwwFormUrlDecoderEntry>> {
                :: std :: convert :: TryInto :: < Collections:: IIterable :: < IWwwFormUrlDecoderEntry > > :: try_into ( self ) . map ( :: windows :: Param :: Owned ) . unwrap_or ( :: windows :: Param :: None )
            }
        }
        impl ::std::convert::TryFrom<WwwFormUrlDecoder>
            for Collections::IVectorView<IWwwFormUrlDecoderEntry>
        {
            type Error = ::windows::Error;
            fn try_from(value: WwwFormUrlDecoder) -> ::windows::Result<Self> {
                ::std::convert::TryFrom::try_from(&value)
            }
        }
        impl ::std::convert::TryFrom<&WwwFormUrlDecoder>
            for Collections::IVectorView<IWwwFormUrlDecoderEntry>
        {
            type Error = ::windows::Error;
            fn try_from(value: &WwwFormUrlDecoder) -> ::windows::Result<Self> {
                ::windows::Interface::cast(value)
            }
        }
        impl<'a> ::windows::IntoParam<'a, Collections::IVectorView<IWwwFormUrlDecoderEntry>>
            for WwwFormUrlDecoder
        {
            fn into_param(
                self,
            ) -> ::windows::Param<'a, Collections::IVectorView<IWwwFormUrlDecoderEntry>>
            {
                ::windows::IntoParam::into_param(&self)
            }
        }
        impl<'a> ::windows::IntoParam<'a, Collections::IVectorView<IWwwFormUrlDecoderEntry>>
            for &WwwFormUrlDecoder
        {
            fn into_param(
                self,
            ) -> ::windows::Param<'a, Collections::IVectorView<IWwwFormUrlDecoderEntry>>
            {
                :: std :: convert :: TryInto :: < Collections:: IVectorView :: < IWwwFormUrlDecoderEntry > > :: try_into ( self ) . map ( :: windows :: Param :: Owned ) . unwrap_or ( :: windows :: Param :: None )
            }
        }
        unsafe impl ::std::marker::Send for WwwFormUrlDecoder {}
        unsafe impl ::std::marker::Sync for WwwFormUrlDecoder {}
        impl ::std::iter::IntoIterator for WwwFormUrlDecoder {
            type Item = IWwwFormUrlDecoderEntry;
            type IntoIter = Collections::VectorViewIterator<Self::Item>;
            fn into_iter(self) -> Self::IntoIter {
                ::std::iter::IntoIterator::into_iter(&self)
            }
        }
        impl ::std::iter::IntoIterator for &WwwFormUrlDecoder {
            type Item = IWwwFormUrlDecoderEntry;
            type IntoIter = Collections::VectorViewIterator<Self::Item>;
            fn into_iter(self) -> Self::IntoIter {
                Collections::VectorViewIterator::new(::std::convert::TryInto::try_into(self).ok())
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Collections {
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IIterable<T>(::windows::IInspectable, ::std::marker::PhantomData<T>)
            where
                T: ::windows::RuntimeType + 'static;
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::Interface for IIterable<T> {
                type Vtable = IIterable_abi<T>;
                const IID: ::windows::Guid = ::windows::Guid::from_signature(
                    <IIterable<T> as ::windows::RuntimeType>::SIGNATURE,
                );
            }
            impl<T: ::windows::RuntimeType + 'static> IIterable<T> {
                pub fn First(&self) -> ::windows::Result<IIterator<T>> {
                    let this = self;
                    unsafe {
                        let mut result__: <IIterator<T> as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::std::mem::transmute_copy(this),
                            &mut result__,
                        )
                        .from_abi::<IIterator<T>>(result__)
                    }
                }
            }
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::RuntimeType for IIterable<T> {
                const SIGNATURE: ::windows::ConstBuffer = {
                    ::windows::ConstBuffer::new()
                        .push_slice(b"pinterface(")
                        .push_slice(b"{faa585ea-6214-4217-afda-7f46de5869b3}")
                        .push_slice(b";")
                        .push_other(<T as ::windows::RuntimeType>::SIGNATURE)
                        .push_slice(b")")
                };
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IIterable<T>>
                for ::windows::IUnknown
            {
                fn from(value: IIterable<T>) -> Self {
                    unsafe { ::std::mem::transmute(value) }
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IIterable<T>>
                for ::windows::IUnknown
            {
                fn from(value: &IIterable<T>) -> Self {
                    ::std::convert::From::from(::std::clone::Clone::clone(value))
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IUnknown> for IIterable<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                    ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IUnknown> for &IIterable<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                    ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IIterable<T>>
                for ::windows::IInspectable
            {
                fn from(value: IIterable<T>) -> Self {
                    value.0
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IIterable<T>>
                for ::windows::IInspectable
            {
                fn from(value: &IIterable<T>) -> Self {
                    value.0.clone()
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IInspectable> for IIterable<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IIterable<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::IntoIterator for IIterable<T> {
                type Item = T;
                type IntoIter = IIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    ::std::iter::IntoIterator::into_iter(&self)
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::IntoIterator for &IIterable<T> {
                type Item = T;
                type IntoIter = IIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    self.First().unwrap()
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IIterable_abi<T>(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub ::std::marker::PhantomData<T>,
            )
            where
                T: ::windows::RuntimeType + 'static;
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IIterator<T>(::windows::IInspectable, ::std::marker::PhantomData<T>)
            where
                T: ::windows::RuntimeType + 'static;
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::Interface for IIterator<T> {
                type Vtable = IIterator_abi<T>;
                const IID: ::windows::Guid = ::windows::Guid::from_signature(
                    <IIterator<T> as ::windows::RuntimeType>::SIGNATURE,
                );
            }
            impl<T: ::windows::RuntimeType + 'static> IIterator<T> {
                pub fn Current(&self) -> ::windows::Result<T> {
                    let this = self;
                    unsafe {
                        let mut result__: <T as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::std::mem::transmute_copy(this),
                            &mut result__,
                        )
                        .from_abi::<T>(result__)
                    }
                }
                pub fn HasCurrent(&self) -> ::windows::Result<bool> {
                    let this = self;
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).7)(
                            ::std::mem::transmute_copy(this),
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn MoveNext(&self) -> ::windows::Result<bool> {
                    let this = self;
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::std::mem::transmute_copy(this),
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn GetMany(
                    &self,
                    items: &mut [<T as ::windows::Abi>::DefaultType],
                ) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).9)(
                            ::std::mem::transmute_copy(this),
                            items.len() as u32,
                            ::std::mem::transmute_copy(&items),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
            }
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::RuntimeType for IIterator<T> {
                const SIGNATURE: ::windows::ConstBuffer = {
                    ::windows::ConstBuffer::new()
                        .push_slice(b"pinterface(")
                        .push_slice(b"{6a79e863-4300-459a-9966-cbb660963ee1}")
                        .push_slice(b";")
                        .push_other(<T as ::windows::RuntimeType>::SIGNATURE)
                        .push_slice(b")")
                };
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IIterator<T>>
                for ::windows::IUnknown
            {
                fn from(value: IIterator<T>) -> Self {
                    unsafe { ::std::mem::transmute(value) }
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IIterator<T>>
                for ::windows::IUnknown
            {
                fn from(value: &IIterator<T>) -> Self {
                    ::std::convert::From::from(::std::clone::Clone::clone(value))
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IUnknown> for IIterator<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                    ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IUnknown> for &IIterator<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                    ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IIterator<T>>
                for ::windows::IInspectable
            {
                fn from(value: IIterator<T>) -> Self {
                    value.0
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IIterator<T>>
                for ::windows::IInspectable
            {
                fn from(value: &IIterator<T>) -> Self {
                    value.0.clone()
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IInspectable> for IIterator<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IIterator<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::Iterator for IIterator<T> {
                type Item = T;
                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    let result = self.Current().ok();
                    if result.is_some() {
                        self.MoveNext().ok()?;
                    }
                    result
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IIterator_abi<T>(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut bool,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut bool,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    items_array_size: u32,
                    items: *mut <T as ::windows::Abi>::Abi,
                    result__: *mut u32,
                ) -> ::windows::HRESULT,
                pub ::std::marker::PhantomData<T>,
            )
            where
                T: ::windows::RuntimeType + 'static;
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IVectorView<T>(::windows::IInspectable, ::std::marker::PhantomData<T>)
            where
                T: ::windows::RuntimeType + 'static;
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::Interface for IVectorView<T> {
                type Vtable = IVectorView_abi<T>;
                const IID: ::windows::Guid = ::windows::Guid::from_signature(
                    <IVectorView<T> as ::windows::RuntimeType>::SIGNATURE,
                );
            }
            impl<T: ::windows::RuntimeType + 'static> IVectorView<T> {
                pub fn GetAt(&self, index: u32) -> ::windows::Result<T> {
                    let this = self;
                    unsafe {
                        let mut result__: <T as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::std::mem::transmute_copy(this),
                            index,
                            &mut result__,
                        )
                        .from_abi::<T>(result__)
                    }
                }
                pub fn Size(&self) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).7)(
                            ::std::mem::transmute_copy(this),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
                pub fn IndexOf<'a, Param0: ::windows::IntoParam<'a, T>>(
                    &self,
                    value: Param0,
                    index: &mut u32,
                ) -> ::windows::Result<bool> {
                    let this = self;
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::std::mem::transmute_copy(this),
                            value.into_param().abi(),
                            index,
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn GetMany(
                    &self,
                    startindex: u32,
                    items: &mut [<T as ::windows::Abi>::DefaultType],
                ) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).9)(
                            ::std::mem::transmute_copy(this),
                            startindex,
                            items.len() as u32,
                            ::std::mem::transmute_copy(&items),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
                pub fn First(&self) -> ::windows::Result<IIterator<T>> {
                    let this = &::windows::Interface::cast::<IIterable<T>>(self)?;
                    unsafe {
                        let mut result__: <IIterator<T> as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::std::mem::transmute_copy(this),
                            &mut result__,
                        )
                        .from_abi::<IIterator<T>>(result__)
                    }
                }
            }
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::RuntimeType for IVectorView<T> {
                const SIGNATURE: ::windows::ConstBuffer = {
                    ::windows::ConstBuffer::new()
                        .push_slice(b"pinterface(")
                        .push_slice(b"{bbe1fa4c-b0e3-4583-baef-1f1b2e483e56}")
                        .push_slice(b";")
                        .push_other(<T as ::windows::RuntimeType>::SIGNATURE)
                        .push_slice(b")")
                };
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IVectorView<T>>
                for ::windows::IUnknown
            {
                fn from(value: IVectorView<T>) -> Self {
                    unsafe { ::std::mem::transmute(value) }
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IVectorView<T>>
                for ::windows::IUnknown
            {
                fn from(value: &IVectorView<T>) -> Self {
                    ::std::convert::From::from(::std::clone::Clone::clone(value))
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IUnknown> for IVectorView<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                    ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(self))
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IUnknown> for &IVectorView<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                    ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IVectorView<T>>
                for ::windows::IInspectable
            {
                fn from(value: IVectorView<T>) -> Self {
                    value.0
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IVectorView<T>>
                for ::windows::IInspectable
            {
                fn from(value: &IVectorView<T>) -> Self {
                    value.0.clone()
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IInspectable> for IVectorView<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::IInspectable> for &'a IVectorView<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::IInspectable> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::TryFrom<IVectorView<T>> for IIterable<T> {
                type Error = ::windows::Error;
                fn try_from(value: IVectorView<T>) -> ::windows::Result<Self> {
                    ::std::convert::TryFrom::try_from(&value)
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::TryFrom<&IVectorView<T>>
                for IIterable<T>
            {
                type Error = ::windows::Error;
                fn try_from(value: &IVectorView<T>) -> ::windows::Result<Self> {
                    ::windows::Interface::cast(value)
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IIterable<T>>
                for IVectorView<T>
            {
                fn into_param(self) -> ::windows::Param<'a, IIterable<T>> {
                    ::windows::IntoParam::into_param(&self)
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IIterable<T>>
                for &IVectorView<T>
            {
                fn into_param(self) -> ::windows::Param<'a, IIterable<T>> {
                    ::std::convert::TryInto::<IIterable<T>>::try_into(self)
                        .map(::windows::Param::Owned)
                        .unwrap_or(::windows::Param::None)
                }
            }
            pub struct VectorViewIterator<T: ::windows::RuntimeType + 'static> {
                vector: ::std::option::Option<IVectorView<T>>,
                current: u32,
            }
            impl<T: ::windows::RuntimeType> VectorViewIterator<T> {
                pub fn new(vector: ::std::option::Option<IVectorView<T>>) -> Self {
                    Self { vector, current: 0 }
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::Iterator for VectorViewIterator<T> {
                type Item = T;
                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    self.vector
                        .as_ref()
                        .and_then(|vector| vector.GetAt(self.current).ok())
                        .and_then(|result| {
                            self.current += 1;
                            Some(result)
                        })
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::IntoIterator for IVectorView<T> {
                type Item = T;
                type IntoIter = VectorViewIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    ::std::iter::IntoIterator::into_iter(&self)
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::IntoIterator for &IVectorView<T> {
                type Item = T;
                type IntoIter = VectorViewIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    VectorViewIterator::new(::std::option::Option::Some(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IVectorView_abi<T>(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    index: u32,
                    result__: *mut <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut u32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: <T as ::windows::Abi>::Abi,
                    index: *mut u32,
                    result__: *mut bool,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    startindex: u32,
                    items_array_size: u32,
                    items: *mut <T as ::windows::Abi>::Abi,
                    result__: *mut u32,
                ) -> ::windows::HRESULT,
                pub ::std::marker::PhantomData<T>,
            )
            where
                T: ::windows::RuntimeType + 'static;
        }
    }
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Storage {
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IStorageFile(::windows::IInspectable);
        unsafe impl ::windows::Interface for IStorageFile {
            type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                4198457734,
                16916,
                17036,
                [166, 76, 20, 201, 172, 115, 21, 234],
            );
        }
        unsafe impl ::windows::RuntimeType for IStorageFile {
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{fa3f6186-4214-428c-a64c-14c9ac7315ea}");
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Streams {
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IBuffer(::windows::IInspectable);
            unsafe impl ::windows::Interface for IBuffer {
                type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    2421821408,
                    48211,
                    4575,
                    [140, 73, 0, 30, 79, 198, 134, 218],
                );
            }
            unsafe impl ::windows::RuntimeType for IBuffer {
                const SIGNATURE: ::windows::ConstBuffer =
                    ::windows::ConstBuffer::from_slice(b"{905a0fe0-bc53-11df-8c49-001e4fc686da}");
            }
        }
    }
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Win32 {
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Foundation {
            #[repr(transparent)]
            #[derive(
                :: std :: default :: Default,
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: fmt :: Debug,
            )]
            pub struct BOOL(pub i32);
            unsafe impl ::windows::Abi for BOOL {
                type Abi = Self;
                type DefaultType = Self;
            }
            impl BOOL {
                #[inline]
                pub fn as_bool(self) -> bool {
                    !(self.0 == 0)
                }
                #[inline]
                pub fn ok(self) -> ::windows::Result<()> {
                    if self.as_bool() {
                        Ok(())
                    } else {
                        Err(::windows::Error::from_win32())
                    }
                }
                #[inline]
                #[track_caller]
                pub fn unwrap(self) {
                    self.ok().unwrap();
                }
                #[inline]
                #[track_caller]
                pub fn expect(self, msg: &str) {
                    self.ok().expect(msg);
                }
            }
            impl ::std::convert::From<BOOL> for bool {
                fn from(value: BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<&BOOL> for bool {
                fn from(value: &BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<bool> for BOOL {
                fn from(value: bool) -> Self {
                    if value {
                        BOOL(1)
                    } else {
                        BOOL(0)
                    }
                }
            }
            impl ::std::convert::From<&bool> for BOOL {
                fn from(value: &bool) -> Self {
                    (*value).into()
                }
            }
            impl ::std::cmp::PartialEq<bool> for BOOL {
                fn eq(&self, other: &bool) -> bool {
                    self.as_bool() == *other
                }
            }
            impl ::std::cmp::PartialEq<BOOL> for bool {
                fn eq(&self, other: &BOOL) -> bool {
                    *self == other.as_bool()
                }
            }
            impl std::ops::Not for BOOL {
                type Output = Self;
                fn not(self) -> Self::Output {
                    if self.as_bool() {
                        BOOL(0)
                    } else {
                        BOOL(1)
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, BOOL> for bool {
                fn into_param(self) -> ::windows::Param<'a, BOOL> {
                    ::windows::Param::Owned(self.into())
                }
            }
            pub unsafe fn CloseHandle<'a, Param0: ::windows::IntoParam<'a, HANDLE>>(
                hobject: Param0,
            ) -> BOOL {
                #[cfg(windows)]
                {
                    #[link(name = "kernel32")]
                    extern "system" {
                        fn CloseHandle(hobject: HANDLE) -> BOOL;
                    }
                    ::std::mem::transmute(CloseHandle(hobject.into_param().abi()))
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: default :: Default,
                :: std :: fmt :: Debug,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
            )]
            #[repr(transparent)]
            pub struct HANDLE(pub isize);
            unsafe impl ::windows::Handle for HANDLE {
                fn is_invalid(&self) -> bool {
                    self.0 == 0 || self.0 == -1
                }
                fn ok(self) -> ::windows::Result<Self> {
                    if self.is_invalid() {
                        Err(::windows::Error::from_win32())
                    } else {
                        Ok(self)
                    }
                }
            }
            unsafe impl ::windows::Abi for HANDLE {
                type Abi = Self;
                type DefaultType = Self;
            }
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: fmt :: Debug,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
            )]
            #[repr(transparent)]
            pub struct HWND(pub isize);
            impl ::std::default::Default for HWND {
                fn default() -> Self {
                    unsafe { ::std::mem::zeroed() }
                }
            }
            unsafe impl ::windows::Handle for HWND {}
            unsafe impl ::windows::Abi for HWND {
                type Abi = Self;
                type DefaultType = Self;
            }
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: fmt :: Debug,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
            )]
            #[repr(transparent)]
            pub struct PSTR(pub *mut u8);
            impl PSTR {
                pub fn is_null(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl ::std::default::Default for PSTR {
                fn default() -> Self {
                    Self(::std::ptr::null_mut())
                }
            }
            unsafe impl ::windows::Abi for PSTR {
                type Abi = Self;
                type DefaultType = Self;
                unsafe fn drop_param(param: &mut ::windows::Param<'_, Self>) {
                    if let ::windows::Param::Boxed(value) = param {
                        if !value.is_null() {
                            unsafe {
                                ::std::boxed::Box::from_raw(value.0);
                            }
                        }
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, PSTR> for &str {
                fn into_param(self) -> ::windows::Param<'a, PSTR> {
                    ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                        self.bytes()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u8>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            impl<'a> ::windows::IntoParam<'a, PSTR> for String {
                fn into_param(self) -> ::windows::Param<'a, PSTR> {
                    ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                        self.bytes()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u8>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: fmt :: Debug,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
            )]
            #[repr(transparent)]
            pub struct PWSTR(pub *mut u16);
            impl PWSTR {
                pub fn is_null(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl ::std::default::Default for PWSTR {
                fn default() -> Self {
                    Self(::std::ptr::null_mut())
                }
            }
            unsafe impl ::windows::Abi for PWSTR {
                type Abi = Self;
                type DefaultType = Self;
                unsafe fn drop_param(param: &mut ::windows::Param<'_, Self>) {
                    if let ::windows::Param::Boxed(value) = param {
                        if !value.is_null() {
                            unsafe {
                                ::std::boxed::Box::from_raw(value.0);
                            }
                        }
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, PWSTR> for &str {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_utf16()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            impl<'a> ::windows::IntoParam<'a, PWSTR> for String {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_utf16()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[cfg(windows)]
            impl<'a> ::windows::IntoParam<'a, PWSTR> for &::std::ffi::OsStr {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    use std::os::windows::ffi::OsStrExt;
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_wide()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[cfg(windows)]
            impl<'a> ::windows::IntoParam<'a, PWSTR> for ::std::ffi::OsString {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    use std::os::windows::ffi::OsStrExt;
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_wide()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Security {
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            #[repr(C)]
            pub struct SECURITY_ATTRIBUTES {
                pub nLength: u32,
                pub lpSecurityDescriptor: *mut ::std::ffi::c_void,
                pub bInheritHandle: super::Foundation::BOOL,
            }
            impl SECURITY_ATTRIBUTES {}
            impl ::std::default::Default for SECURITY_ATTRIBUTES {
                fn default() -> Self {
                    unsafe { ::std::mem::zeroed() }
                }
            }
            impl ::std::fmt::Debug for SECURITY_ATTRIBUTES {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("SECURITY_ATTRIBUTES")
                        .field("nLength", &self.nLength)
                        .field("lpSecurityDescriptor", &self.lpSecurityDescriptor)
                        .field("bInheritHandle", &self.bInheritHandle)
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for SECURITY_ATTRIBUTES {
                fn eq(&self, other: &Self) -> bool {
                    self.nLength == other.nLength
                        && self.lpSecurityDescriptor == other.lpSecurityDescriptor
                        && self.bInheritHandle == other.bInheritHandle
                }
            }
            impl ::std::cmp::Eq for SECURITY_ATTRIBUTES {}
            unsafe impl ::windows::Abi for SECURITY_ATTRIBUTES {
                type Abi = Self;
                type DefaultType = Self;
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod System {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Threading {
                pub unsafe fn CreateEventW<
                    'a,
                    Param1: ::windows::IntoParam<'a, super::super::Foundation::BOOL>,
                    Param2: ::windows::IntoParam<'a, super::super::Foundation::BOOL>,
                    Param3: ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                >(
                    lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                    bmanualreset: Param1,
                    binitialstate: Param2,
                    lpname: Param3,
                ) -> super::super::Foundation::HANDLE {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn CreateEventW(
                                lpeventattributes : * const super::super::Security:: SECURITY_ATTRIBUTES,
                                bmanualreset: super::super::Foundation::BOOL,
                                binitialstate: super::super::Foundation::BOOL,
                                lpname: super::super::Foundation::PWSTR,
                            ) -> super::super::Foundation::HANDLE;
                        }
                        ::std::mem::transmute(CreateEventW(
                            ::std::mem::transmute(lpeventattributes),
                            bmanualreset.into_param().abi(),
                            binitialstate.into_param().abi(),
                            lpname.into_param().abi(),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn SetEvent<
                    'a,
                    Param0: ::windows::IntoParam<'a, super::super::Foundation::HANDLE>,
                >(
                    hevent: Param0,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn SetEvent(
                                hevent: super::super::Foundation::HANDLE,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(SetEvent(hevent.into_param().abi()))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct WAIT_RETURN_CAUSE(pub u32);
                pub const WAIT_OBJECT_0: WAIT_RETURN_CAUSE = WAIT_RETURN_CAUSE(0u32);
                pub const WAIT_ABANDONED: WAIT_RETURN_CAUSE = WAIT_RETURN_CAUSE(128u32);
                pub const WAIT_ABANDONED_0: WAIT_RETURN_CAUSE = WAIT_RETURN_CAUSE(128u32);
                pub const WAIT_IO_COMPLETION: WAIT_RETURN_CAUSE = WAIT_RETURN_CAUSE(192u32);
                pub const WAIT_TIMEOUT: WAIT_RETURN_CAUSE = WAIT_RETURN_CAUSE(258u32);
                pub const WAIT_FAILED: WAIT_RETURN_CAUSE = WAIT_RETURN_CAUSE(4294967295u32);
                impl ::std::convert::From<u32> for WAIT_RETURN_CAUSE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for WAIT_RETURN_CAUSE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for WAIT_RETURN_CAUSE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for WAIT_RETURN_CAUSE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for WAIT_RETURN_CAUSE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for WAIT_RETURN_CAUSE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                impl ::std::ops::Not for WAIT_RETURN_CAUSE {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                pub unsafe fn WaitForSingleObject<
                    'a,
                    Param0: ::windows::IntoParam<'a, super::super::Foundation::HANDLE>,
                >(
                    hhandle: Param0,
                    dwmilliseconds: u32,
                ) -> WAIT_RETURN_CAUSE {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn WaitForSingleObject(
                                hhandle: super::super::Foundation::HANDLE,
                                dwmilliseconds: u32,
                            ) -> WAIT_RETURN_CAUSE;
                        }
                        ::std::mem::transmute(WaitForSingleObject(
                            hhandle.into_param().abi(),
                            ::std::mem::transmute(dwmilliseconds),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod UI {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod WindowsAndMessaging {
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct MESSAGEBOX_RESULT(pub i32);
                pub const IDOK: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(1i32);
                pub const IDCANCEL: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(2i32);
                pub const IDABORT: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(3i32);
                pub const IDRETRY: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(4i32);
                pub const IDIGNORE: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(5i32);
                pub const IDYES: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(6i32);
                pub const IDNO: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(7i32);
                pub const IDCLOSE: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(8i32);
                pub const IDHELP: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(9i32);
                pub const IDTRYAGAIN: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(10i32);
                pub const IDCONTINUE: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(11i32);
                pub const IDASYNC: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(32001i32);
                pub const IDTIMEOUT: MESSAGEBOX_RESULT = MESSAGEBOX_RESULT(32000i32);
                impl ::std::convert::From<i32> for MESSAGEBOX_RESULT {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for MESSAGEBOX_RESULT {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct MESSAGEBOX_STYLE(pub u32);
                pub const MB_ABORTRETRYIGNORE: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(2u32);
                pub const MB_CANCELTRYCONTINUE: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(6u32);
                pub const MB_HELP: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16384u32);
                pub const MB_OK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(0u32);
                pub const MB_OKCANCEL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(1u32);
                pub const MB_RETRYCANCEL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(5u32);
                pub const MB_YESNO: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(4u32);
                pub const MB_YESNOCANCEL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(3u32);
                pub const MB_ICONHAND: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16u32);
                pub const MB_ICONQUESTION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(32u32);
                pub const MB_ICONEXCLAMATION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(48u32);
                pub const MB_ICONASTERISK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(64u32);
                pub const MB_USERICON: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(128u32);
                pub const MB_ICONWARNING: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(48u32);
                pub const MB_ICONERROR: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16u32);
                pub const MB_ICONINFORMATION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(64u32);
                pub const MB_ICONSTOP: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(16u32);
                pub const MB_DEFBUTTON1: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(0u32);
                pub const MB_DEFBUTTON2: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(256u32);
                pub const MB_DEFBUTTON3: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(512u32);
                pub const MB_DEFBUTTON4: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(768u32);
                pub const MB_APPLMODAL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(0u32);
                pub const MB_SYSTEMMODAL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(4096u32);
                pub const MB_TASKMODAL: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(8192u32);
                pub const MB_NOFOCUS: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(32768u32);
                pub const MB_SETFOREGROUND: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(65536u32);
                pub const MB_DEFAULT_DESKTOP_ONLY: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(131072u32);
                pub const MB_TOPMOST: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(262144u32);
                pub const MB_RIGHT: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(524288u32);
                pub const MB_RTLREADING: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(1048576u32);
                pub const MB_SERVICE_NOTIFICATION: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(2097152u32);
                pub const MB_SERVICE_NOTIFICATION_NT3X: MESSAGEBOX_STYLE =
                    MESSAGEBOX_STYLE(262144u32);
                pub const MB_TYPEMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(15u32);
                pub const MB_ICONMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(240u32);
                pub const MB_DEFMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(3840u32);
                pub const MB_MODEMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(12288u32);
                pub const MB_MISCMASK: MESSAGEBOX_STYLE = MESSAGEBOX_STYLE(49152u32);
                impl ::std::convert::From<u32> for MESSAGEBOX_STYLE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for MESSAGEBOX_STYLE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for MESSAGEBOX_STYLE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for MESSAGEBOX_STYLE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for MESSAGEBOX_STYLE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for MESSAGEBOX_STYLE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                impl ::std::ops::Not for MESSAGEBOX_STYLE {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                pub unsafe fn MessageBoxA<
                    'a,
                    Param0: ::windows::IntoParam<'a, super::super::Foundation::HWND>,
                    Param1: ::windows::IntoParam<'a, super::super::Foundation::PSTR>,
                    Param2: ::windows::IntoParam<'a, super::super::Foundation::PSTR>,
                >(
                    hwnd: Param0,
                    lptext: Param1,
                    lpcaption: Param2,
                    utype: MESSAGEBOX_STYLE,
                ) -> MESSAGEBOX_RESULT {
                    #[cfg(windows)]
                    {
                        #[link(name = "user32")]
                        extern "system" {
                            fn MessageBoxA(
                                hwnd: super::super::Foundation::HWND,
                                lptext: super::super::Foundation::PSTR,
                                lpcaption: super::super::Foundation::PSTR,
                                utype: MESSAGEBOX_STYLE,
                            ) -> MESSAGEBOX_RESULT;
                        }
                        ::std::mem::transmute(MessageBoxA(
                            hwnd.into_param().abi(),
                            lptext.into_param().abi(),
                            lpcaption.into_param().abi(),
                            ::std::mem::transmute(utype),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
    }
}
