#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
pub use pallet::*;
///
///			The module that hosts all the
///			[FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame)
///			types needed to add this pallet to a
///			[runtime](https://substrate.dev/docs/en/knowledgebase/runtime/).
///
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;
    ///
    ///			Configuration trait of this pallet.
    ///
    ///			Implement this type for a runtime in order to customize this pallet.
    ///
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }
    ///
    ///			The [event](https://substrate.dev/docs/en/knowledgebase/runtime/events) emitted
    ///			by this pallet.
    ///
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    pub enum Event<T: Config> {
        /// Event emitted when a proof has been claimed. [who, claim]
        ClaimCreated(T::AccountId, Vec<u8>),
        /// Event emitted when a claim is revoked by the owner. [who, claim]
        ClaimRevoked(T::AccountId, Vec<u8>),
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T)>,
            frame_support::Never,
        ),
    }
    const _: () = {
        impl<T: Config> core::clone::Clone for Event<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::ClaimCreated(ref _0, ref _1) => Self::ClaimCreated(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                    ),
                    Self::ClaimRevoked(ref _0, ref _1) => Self::ClaimRevoked(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                    ),
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(core::clone::Clone::clone(_0), core::clone::Clone::clone(_1))
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Event<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Event<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::ClaimCreated(_0, _1), Self::ClaimCreated(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::ClaimRevoked(_0, _1), Self::ClaimRevoked(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::ClaimCreated { .. }, Self::ClaimRevoked { .. }) => false,
                    (Self::ClaimCreated { .. }, Self::__Ignore { .. }) => false,
                    (Self::ClaimRevoked { .. }, Self::ClaimCreated { .. }) => false,
                    (Self::ClaimRevoked { .. }, Self::__Ignore { .. }) => false,
                    (Self::__Ignore { .. }, Self::ClaimCreated { .. }) => false,
                    (Self::__Ignore { .. }, Self::ClaimRevoked { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::fmt::Debug for Event<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::ClaimCreated(ref _0, ref _1) => fmt
                        .debug_tuple("Event::ClaimCreated")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::ClaimRevoked(ref _0, ref _1) => fmt
                        .debug_tuple("Event::ClaimRevoked")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::__Ignore(ref _0, ref _1) => fmt
                        .debug_tuple("Event::__Ignore")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> ::codec::Encode for Event<T>
        where
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
        {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Event::ClaimCreated(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    Event::ClaimRevoked(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                        ::codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl<T: Config> ::codec::EncodeLike for Event<T>
        where
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
        {
        }
    };
    const _: () = {
        impl<T: Config> ::codec::Decode for Event<T>
        where
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
        {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Event`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::ClaimCreated(
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::ClaimCreated.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Vec<u8> as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::ClaimCreated.1`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        ))
                    }
                    __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::ClaimRevoked(
                            {
                                let __codec_res_edqy =
                                    <T::AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::ClaimRevoked.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Vec<u8> as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::ClaimRevoked.1`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        ))
                    }
                    _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                        "Could not decode `Event`, variant doesn\'t exist",
                    )),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Event<T>
        where
            T::AccountId: ::scale_info::TypeInfo + 'static,
            T::AccountId: ::scale_info::TypeInfo + 'static,
            frame_support::sp_std::marker::PhantomData<(T)>: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                :: scale_info :: Type :: builder () . path (:: scale_info :: Path :: new ("Event" , "pallet_template::pallet")) . type_params (< [_] > :: into_vec (box [:: scale_info :: TypeParameter :: new ("T" , :: core :: option :: Option :: None)])) . docs_always (& ["\n\t\t\tThe [event](https://substrate.dev/docs/en/knowledgebase/runtime/events) emitted\n\t\t\tby this pallet.\n\t\t\t"]) . variant (:: scale_info :: build :: Variants :: new () . variant ("ClaimCreated" , | v | v . index (0usize as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < T :: AccountId > () . type_name ("T::AccountId") . docs_always (& [])) . field (| f | f . ty :: < Vec < u8 > > () . type_name ("Vec<u8>") . docs_always (& []))) . docs_always (& ["Event emitted when a proof has been claimed. [who, claim]"])) . variant ("ClaimRevoked" , | v | v . index (1usize as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < T :: AccountId > () . type_name ("T::AccountId") . docs_always (& [])) . field (| f | f . ty :: < Vec < u8 > > () . type_name ("Vec<u8>") . docs_always (& []))) . docs_always (& ["Event emitted when a claim is revoked by the owner. [who, claim]"])))
            }
        };
    };
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    ///
    ///			Custom [dispatch errors](https://substrate.dev/docs/en/knowledgebase/runtime/errors)
    ///			of this pallet.
    ///
    pub enum Error<T> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T)>,
            frame_support::Never,
        ),
        /// The proof has already been claimed.
        ProofAlreadyClaimed,
        /// The proof does not exist, so it cannot be revoked.
        NoSuchProof,
        /// The proof is claimed by another account, so caller can't revoke it.
        NotProofOwner,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T> ::scale_info::TypeInfo for Error<T>
        where
            frame_support::sp_std::marker::PhantomData<(T)>: ::scale_info::TypeInfo + 'static,
            T: 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                :: scale_info :: Type :: builder () . path (:: scale_info :: Path :: new ("Error" , "pallet_template::pallet")) . type_params (< [_] > :: into_vec (box [:: scale_info :: TypeParameter :: new ("T" , :: core :: option :: Option :: None)])) . docs_always (& ["\n\t\t\tCustom [dispatch errors](https://substrate.dev/docs/en/knowledgebase/runtime/errors)\n\t\t\tof this pallet.\n\t\t\t"]) . variant (:: scale_info :: build :: Variants :: new () . variant ("ProofAlreadyClaimed" , | v | v . index (0usize as :: core :: primitive :: u8) . docs_always (& ["The proof has already been claimed."])) . variant ("NoSuchProof" , | v | v . index (1usize as :: core :: primitive :: u8) . docs_always (& ["The proof does not exist, so it cannot be revoked."])) . variant ("NotProofOwner" , | v | v . index (2usize as :: core :: primitive :: u8) . docs_always (& ["The proof is claimed by another account, so caller can\'t revoke it."])))
            }
        };
    };
    ///
    ///			The [pallet](https://substrate.dev/docs/en/knowledgebase/runtime/pallets) implementing
    ///			the on-chain logic.
    ///
    pub struct Pallet<T>(frame_support::sp_std::marker::PhantomData<(T)>);
    const _: () = {
        impl<T> core::clone::Clone for Pallet<T> {
            fn clone(&self) -> Self {
                Self(core::clone::Clone::clone(&self.0))
            }
        }
    };
    const _: () = {
        impl<T> core::cmp::Eq for Pallet<T> {}
    };
    const _: () = {
        impl<T> core::cmp::PartialEq for Pallet<T> {
            fn eq(&self, other: &Self) -> bool {
                true && self.0 == other.0
            }
        }
    };
    const _: () = {
        impl<T> core::fmt::Debug for Pallet<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Pallet").field(&self.0).finish()
            }
        }
    };
    #[allow(type_alias_bounds)]
    pub(super) type Proofs<T: Config> = StorageMap<
        _GeneratedPrefixForStorageProofs<T>,
        Blake2_128Concat,
        Vec<u8>,
        (T::AccountId, T::BlockNumber),
        ValueQuery,
    >;
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    impl<T: Config> Pallet<T> {
        pub fn create_claim(origin: OriginFor<T>, proof: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            {
                if !!Proofs::<T>::contains_key(&proof) {
                    {
                        return Err(Error::<T>::ProofAlreadyClaimed.into());
                    };
                }
            };
            let current_block = <frame_system::Pallet<T>>::block_number();
            Proofs::<T>::insert(&proof, (&sender, current_block));
            Self::deposit_event(Event::ClaimCreated(sender, proof));
            Ok(())
        }
        pub fn revoke_claim(origin: OriginFor<T>, proof: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            {
                if !Proofs::<T>::contains_key(&proof) {
                    {
                        return Err(Error::<T>::NoSuchProof.into());
                    };
                }
            };
            let (owner, _) = Proofs::<T>::get(&proof);
            {
                if !(sender == owner) {
                    {
                        return Err(Error::<T>::NotProofOwner.into());
                    };
                }
            };
            Proofs::<T>::remove(&proof);
            Self::deposit_event(Event::ClaimRevoked(sender, proof));
            Ok(())
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_constants_metadata(
        ) -> frame_support::sp_std::vec::Vec<frame_support::metadata::PalletConstantMetadata>
        {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn error_metadata() -> Option<frame_support::metadata::PalletErrorMetadata> {
            Some(frame_support::metadata::PalletErrorMetadata {
                ty: frame_support::scale_info::meta_type::<Error<T>>(),
            })
        }
    }
    /// Type alias to `Pallet`, to be used by `construct_runtime`.
    ///
    /// Generated by `pallet` attribute macro.
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl<T: Config> frame_support::traits::GetStorageVersion for Pallet<T> {
        fn current_storage_version() -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::default()
        }
        fn on_chain_storage_version() -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::get::<Self>()
        }
    }
    impl<T: Config> frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            let storage_version = frame_support::traits::StorageVersion::default();
            storage_version.put::<Self>();
        }
    }
    impl<T: Config> frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::index::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
        fn name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
        fn module_name() -> &'static str {
            < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: module_name :: < Self > () . expect ("Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime")
        }
        fn crate_version() -> frame_support::traits::CrateVersion {
            frame_support::traits::CrateVersion {
                major: 3u16,
                minor: 0u8,
                patch: 0u8,
            }
        }
    }
    impl<T: Config> frame_support::traits::StorageInfoTrait for Pallet<T> {
        fn storage_info() -> frame_support::sp_std::vec::Vec<frame_support::traits::StorageInfo> {
            #[allow(unused_mut)]
            let mut res = ::alloc::vec::Vec::new();
            {
                let mut storage_info = < Proofs < T > as frame_support :: traits :: PartialStorageInfoTrait > :: partial_storage_info () ;
                res.append(&mut storage_info);
            }
            res
        }
    }
    #[doc(hidden)]
    pub mod __substrate_call_check {
        #[doc(hidden)]
        pub use __is_call_part_defined_0 as is_call_part_defined;
    }
    ///Contains one variant per dispatchable that can be called by an extrinsic.
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T,)>,
            frame_support::Never,
        ),
        create_claim {
            proof: Vec<u8>,
        },
        revoke_claim {
            proof: Vec<u8>,
        },
    }
    const _: () = {
        impl<T: Config> core::fmt::Debug for Call<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::__Ignore(ref _0, ref _1) => fmt
                        .debug_tuple("Call::__Ignore")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::create_claim { ref proof } => fmt
                        .debug_struct("Call::create_claim")
                        .field("proof", &proof)
                        .finish(),
                    Self::revoke_claim { ref proof } => fmt
                        .debug_struct("Call::revoke_claim")
                        .field("proof", &proof)
                        .finish(),
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::clone::Clone for Call<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(core::clone::Clone::clone(_0), core::clone::Clone::clone(_1))
                    }
                    Self::create_claim { ref proof } => Self::create_claim {
                        proof: core::clone::Clone::clone(proof),
                    },
                    Self::revoke_claim { ref proof } => Self::revoke_claim {
                        proof: core::clone::Clone::clone(proof),
                    },
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Call<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Call<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::create_claim { proof }, Self::create_claim { proof: _0 }) => {
                        true && proof == _0
                    }
                    (Self::revoke_claim { proof }, Self::revoke_claim { proof: _0 }) => {
                        true && proof == _0
                    }
                    (Self::__Ignore { .. }, Self::create_claim { .. }) => false,
                    (Self::__Ignore { .. }, Self::revoke_claim { .. }) => false,
                    (Self::create_claim { .. }, Self::__Ignore { .. }) => false,
                    (Self::create_claim { .. }, Self::revoke_claim { .. }) => false,
                    (Self::revoke_claim { .. }, Self::__Ignore { .. }) => false,
                    (Self::revoke_claim { .. }, Self::create_claim { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        #[allow(non_camel_case_types)]
        impl<T: Config> ::codec::Encode for Call<T> {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Call::create_claim { ref proof } => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(proof, __codec_dest_edqy);
                    }
                    Call::revoke_claim { ref proof } => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(proof, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl<T: Config> ::codec::EncodeLike for Call<T> {}
    };
    const _: () = {
        #[allow(non_camel_case_types)]
        impl<T: Config> ::codec::Decode for Call<T> {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::create_claim {
                            proof: {
                                let __codec_res_edqy =
                                    <Vec<u8> as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::create_claim::proof`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::revoke_claim {
                            proof: {
                                let __codec_res_edqy =
                                    <Vec<u8> as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::revoke_claim::proof`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                        "Could not decode `Call`, variant doesn\'t exist",
                    )),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Call<T>
        where
            frame_support::sp_std::marker::PhantomData<(T,)>: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Call", "pallet_template::pallet"))
                    .type_params(<[_]>::into_vec(box [::scale_info::TypeParameter::new(
                        "T",
                        ::core::option::Option::None,
                    )]))
                    .docs_always(&[
                        "Contains one variant per dispatchable that can be called by an extrinsic.",
                    ])
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant("create_claim", |v| {
                                v.index(0usize as ::core::primitive::u8)
                                    .fields(::scale_info::build::Fields::named().field(|f| {
                                        f.ty::<Vec<u8>>()
                                            .name("proof")
                                            .type_name("Vec<u8>")
                                            .docs_always(&[])
                                    }))
                                    .docs_always(&[])
                            })
                            .variant("revoke_claim", |v| {
                                v.index(1usize as ::core::primitive::u8)
                                    .fields(::scale_info::build::Fields::named().field(|f| {
                                        f.ty::<Vec<u8>>()
                                            .name("proof")
                                            .type_name("Vec<u8>")
                                            .docs_always(&[])
                                    }))
                                    .docs_always(&[])
                            }),
                    )
            }
        };
    };
    impl<T: Config> Call<T> {
        ///Create a call with the variant `create_claim`.
        pub fn new_call_variant_create_claim(proof: Vec<u8>) -> Self {
            Self::create_claim { proof }
        }
        ///Create a call with the variant `revoke_claim`.
        pub fn new_call_variant_revoke_claim(proof: Vec<u8>) -> Self {
            Self::revoke_claim { proof }
        }
    }
    impl<T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
            match *self {
                Self::create_claim { ref proof } => {
                    let __pallet_base_weight = 1_000;
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<(&Vec<u8>,)>>::weigh_data(
                            &__pallet_base_weight,
                            (proof,),
                        );
                    let __pallet_class = < dyn frame_support :: dispatch :: ClassifyDispatch < (& Vec < u8 > ,) > > :: classify_dispatch (& __pallet_base_weight , (proof ,)) ;
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&Vec<u8>,)>>::pays_fee(
                            &__pallet_base_weight,
                            (proof,),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::revoke_claim { ref proof } => {
                    let __pallet_base_weight = 10_000;
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<(&Vec<u8>,)>>::weigh_data(
                            &__pallet_base_weight,
                            (proof,),
                        );
                    let __pallet_class = < dyn frame_support :: dispatch :: ClassifyDispatch < (& Vec < u8 > ,) > > :: classify_dispatch (& __pallet_base_weight , (proof ,)) ;
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&Vec<u8>,)>>::pays_fee(
                            &__pallet_base_weight,
                            (proof,),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::__Ignore(_, _) => ::core::panicking::panic_fmt(
                    match match (&"__Ignore cannot be used",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe {
                            ::core::fmt::Arguments::new_v1(
                                &["internal error: entered unreachable code: "],
                                args,
                            )
                        },
                    },
                ),
            }
        }
    }
    impl<T: Config> frame_support::dispatch::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::create_claim { .. } => "create_claim",
                Self::revoke_claim { .. } => "revoke_claim",
                Self::__Ignore(_, _) => ::core::panicking::panic_fmt(
                    match match (&"__PhantomItem cannot be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe {
                            ::core::fmt::Arguments::new_v1(
                                &["internal error: entered unreachable code: "],
                                args,
                            )
                        },
                    },
                ),
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &["create_claim", "revoke_claim"]
        }
    }
    impl<T: Config> frame_support::traits::UnfilteredDispatchable for Call<T> {
        type Origin = frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(
            self,
            origin: Self::Origin,
        ) -> frame_support::dispatch::DispatchResultWithPostInfo {
            match self {
                Self::create_claim { proof } => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "create_claim",
                                    "pallet_template::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/template/src/lib.rs"),
                                    Some(6u32),
                                    Some("pallet_template::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span = CALLSITE.disabled_span();
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::create_claim(origin, proof)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::revoke_claim { proof } => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "revoke_claim",
                                    "pallet_template::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("pallets/template/src/lib.rs"),
                                    Some(6u32),
                                    Some("pallet_template::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span = CALLSITE.disabled_span();
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::revoke_claim(origin, proof)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::__Ignore(_, _) => {
                    let _ = origin;
                    {
                        {
                            ::core::panicking::panic_fmt(
                                match match (&"__PhantomItem cannot be used.",) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                } {
                                    ref args => unsafe {
                                        ::core::fmt::Arguments::new_v1(
                                            &["internal error: entered unreachable code: "],
                                            args,
                                        )
                                    },
                                },
                            )
                        }
                    };
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::Callable<T> for Pallet<T> {
        type Call = Call<T>;
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn call_functions() -> frame_support::metadata::PalletCallMetadata {
            frame_support::scale_info::meta_type::<Call<T>>().into()
        }
    }
    impl<T: Config> frame_support::sp_std::fmt::Debug for Error<T> {
        fn fmt(
            &self,
            f: &mut frame_support::sp_std::fmt::Formatter<'_>,
        ) -> frame_support::sp_std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl<T: Config> Error<T> {
        pub fn as_u8(&self) -> u8 {
            match &self {
                Self::__Ignore(_, _) => ::core::panicking::panic_fmt(
                    match match (&"`__Ignore` can never be constructed",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe {
                            ::core::fmt::Arguments::new_v1(
                                &["internal error: entered unreachable code: "],
                                args,
                            )
                        },
                    },
                ),
                Self::ProofAlreadyClaimed => 0usize as u8,
                Self::NoSuchProof => 1usize as u8,
                Self::NotProofOwner => 2usize as u8,
            }
        }
        pub fn as_str(&self) -> &'static str {
            match &self {
                Self::__Ignore(_, _) => ::core::panicking::panic_fmt(
                    match match (&"`__Ignore` can never be constructed",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    } {
                        ref args => unsafe {
                            ::core::fmt::Arguments::new_v1(
                                &["internal error: entered unreachable code: "],
                                args,
                            )
                        },
                    },
                ),
                Self::ProofAlreadyClaimed => "ProofAlreadyClaimed",
                Self::NoSuchProof => "NoSuchProof",
                Self::NotProofOwner => "NotProofOwner",
            }
        }
    }
    impl<T: Config> From<Error<T>> for &'static str {
        fn from(err: Error<T>) -> &'static str {
            err.as_str()
        }
    }
    impl<T: Config> From<Error<T>> for frame_support::sp_runtime::DispatchError {
        fn from(err: Error<T>) -> Self {
            let index = < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: index :: < Pallet < T > > () . expect ("Every active module has an index in the runtime; qed") as u8 ;
            frame_support::sp_runtime::DispatchError::Module {
                index,
                error: err.as_u8(),
                message: Some(err.as_str()),
            }
        }
    }
    #[doc(hidden)]
    pub mod __substrate_event_check {
        #[doc(hidden)]
        pub use __is_event_part_defined_1 as is_event_part_defined;
    }
    impl<T: Config> Pallet<T> {
        pub(super) fn deposit_event(event: Event<T>) {
            let event = <<T as Config>::Event as From<Event<T>>>::from(event);
            let event =
                <<T as Config>::Event as Into<<T as frame_system::Config>::Event>>::into(event);
            <frame_system::Pallet<T>>::deposit_event(event)
        }
    }
    impl<T: Config> From<Event<T>> for () {
        fn from(_: Event<T>) {}
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata() -> frame_support::metadata::PalletStorageMetadata {
            frame_support :: metadata :: PalletStorageMetadata { prefix : < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Pallet < T > > () . expect ("Every active pallet has a name in the runtime; qed") , entries : { # [allow (unused_mut)] let mut entries = :: alloc :: vec :: Vec :: new () ; { < Proofs < T > as frame_support :: storage :: StorageEntryMetadataBuilder > :: build_metadata (:: alloc :: vec :: Vec :: new () , & mut entries) ; } entries } , }
        }
    }
    pub(super) struct _GeneratedPrefixForStorageProofs<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance for _GeneratedPrefixForStorageProofs<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
            .expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "Proofs";
    }
    #[doc(hidden)]
    pub mod __substrate_inherent_check {
        #[doc(hidden)]
        pub use __is_inherent_part_defined_2 as is_inherent_part_defined;
    }
    /// Hidden instance generated to be internally used when module is used without
    /// instance.
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    pub(super) trait Store {
        type Proofs;
    }
    impl<T: Config> Store for Pallet<T> {
        type Proofs = Proofs<T>;
    }
    impl<T: Config> frame_support::traits::OnFinalize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_finalize(n: <T as frame_system::Config>::BlockNumber) {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_finalize",
                            "pallet_template::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/template/src/lib.rs"),
                            Some(6u32),
                            Some("pallet_template::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_finalize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnIdle<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_idle(
            n: <T as frame_system::Config>::BlockNumber,
            remaining_weight: frame_support::weights::Weight,
        ) -> frame_support::weights::Weight {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_idle (n , remaining_weight)
        }
    }
    impl<T: Config> frame_support::traits::OnInitialize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_initialize(
            n: <T as frame_system::Config>::BlockNumber,
        ) -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_initialize",
                            "pallet_template::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/template/src/lib.rs"),
                            Some(6u32),
                            Some("pallet_template::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_initialize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_runtime_update",
                            "pallet_template::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/template/src/lib.rs"),
                            Some(6u32),
                            Some("pallet_template::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            let pallet_name = < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Self > () . unwrap_or ("<unknown pallet name>") ;
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        match match (&pallet_name,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        } {
                            ref args => unsafe {
                                ::core::fmt::Arguments::new_v1(
                                    &["\u{2705} no migration for "],
                                    args,
                                )
                            },
                        },
                        lvl,
                        &(
                            frame_support::LOG_TARGET,
                            "pallet_template::pallet",
                            "pallets/template/src/lib.rs",
                            6u32,
                        ),
                    );
                }
            };
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_runtime_upgrade ()
        }
    }
    impl<T: Config> frame_support::traits::OffchainWorker<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn offchain_worker(n: <T as frame_system::Config>::BlockNumber) {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: offchain_worker (n)
        }
    }
    impl<T: Config> frame_support::traits::IntegrityTest for Pallet<T> {
        fn integrity_test() {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: integrity_test ()
        }
    }
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[doc(hidden)]
        pub use __is_genesis_config_defined_3 as is_genesis_config_defined;
        #[doc(hidden)]
        pub use __is_std_enabled_for_genesis_3 as is_std_enabled_for_genesis;
    }
    #[doc(hidden)]
    pub mod __substrate_origin_check {
        #[doc(hidden)]
        pub use __is_origin_part_defined_4 as is_origin_part_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_validate_unsigned_check {
        #[doc(hidden)]
        pub use __is_validate_unsigned_part_defined_5 as is_validate_unsigned_part_defined;
    }
}
