#![allow(missing_docs)]
use std::any::{Any,TypeId};
use std::cmp;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use typenum::{self, Diff, Max, Maximum, Min, Minimum, Prod, Quot, Sum, Unsigned};

#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone,Copy,Eq,PartialEq,Debug)]
#[cfg_attr(all(not(target_os = "cuda"),feature = "cuda"),
  derive(cust::DeviceCopy)
)]
pub struct Dynamic{
    value :usize,
}

impl Dynamic{
    #[inline]
    pub const fn new(value :usize) ->Self{
        Self{value}
    }
}
#[cfg(feature = "serde-serialize-no-std")]
impl Serialize for Dynamic{
    fn serialize<S>(&self, serializer:S) -> Result<S::Ok,S::Error>
    where
        S: Serializer,{
        self.value.serialize(serializer)
    }
}
#[cfg(feature = "serde-serialize-no-std")]
impl<'de> Deserialize<'de> for Dynamic{
    fn deserialize<D>(deserializer:D)-> Result<Self,D::Error>
    where
         D:Deserializer<'de>,
    {
        usize::deserialize(deserializer).map(|x| Dynamic{value:x})
    }
}

pub trait IsDynamic{}
pub trait  IsNotStaticOne{}

impl IsDynamic for Dynamic{}
impl IsNotStaticOne for Dynamic{}

pub unsafe trait Dim: Any+Debug+Copy+PartialEq+Send+Sync{
    #[inline(always)]
    fn is<D:Dim>() -> bool{
        TypeId::of::<Self>() == TypeId::of::<D>()
    }

    fn try_to_usize() -> Option<usize>;

    fn value(&self) -> usize;

    fn from_usize(dim:usize) -> Self;
}

unsafe impl Dim for Dynamic{
    #[inline]
    fn try_to_usize() -> Option<usize> {
        None
    }
    #[inline]
    fn from_usize(dim: usize) -> Self {
        Self::new(dim)
    }
    #[inline]
    fn value(&self) -> usize {
       self.value
    }
}

impl Add<usize> for Dynamic {
    type Output = Dynamic;

    #[inline]
    fn add(self, rhs:usize)->Self {
        Self::new(self.value + rhs)
    }
}

impl Sub<usize> for Dynamic{
    type Output = Dynamic;
    #[inline]
    fn sub(self,rhs:usize)->Self{
        Self::new(self.value-rhs)
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    all(not(target_os = "cuda"), feature = "cuda"),
    derive(cust::DeviceCopy)
)]
pub struct  Const<const  R:usize>;

pub trait  DimName:Dim{
    const USIZE: usize;
    fn name()->Self;
    fn dim() ->usize;
}
#[cfg(feature = "serde-serialize-no-std")]
impl <const D:usize> Serialize for Const<D> {
    fn serialize<S>(&self, serializer:S) -> Result<S::Ok,S::Error>
    where
    S:Serializer,
    {
        ().serialize(serializer)
    }
}

#[cfg(feature = "serde-serialize-no-std")]
impl <'de,const D:usize> Deserialize<'de> for Const<D> {
    fn deserialize<Des>(deserializer:Des) -> Result<Self,Des::Error>
    where
       Des: Deseriailizer<'de>,
    {
        <()>::deserialize(deserializer).map(|_| Const::<D>)
    }
}

#[cfg(feature = "rkyv-serialize-no-std")]
mod rkyv_impl{
    use super::Const;
    use rkyv::{Archive,Deserialize,Fallible,Serialize};

    impl<const R:usize> Archive for Const<R>{
        type Archived = Self;
        type Resolver = ();

        fn resolve(
            &self,
            _:usize,
            _: Self::Resolver,
            _: &mut core::mem::Maybeinit<self::Archived>,
        ) {
        }
    }
    impl<S:Fallible +?Sized,const R:usize> Serialized<S> for Const<R>{
        fn serialize(&self,_:&mut S) -> Result<Self::Resolver,S::Error>{
            Ok(())
        }
    }
     impl<D: Fallible + ?Sized, const R: usize> Deserialize<Self, D> for Const<R> {
        fn deserialize(&self, _: &mut D) -> Result<Self, D::Error> {
            Ok(Const)
        }
    }
}

pub trait ToConst{
    type Const:DimName;
}

pub trait ToTypenum{
    type  Typenum:unsigned;
}

unsafe impl<const T:usize>Dim for Const<T>{
    fn try_to_usize() -> Option<usize> {
       Some(T)
    }
    fn value(&self) -> usize {
       T
    }
    fn from_usize(dim: usize) -> Self {
       assert_eq!(dim,T);
        Self
    }
}