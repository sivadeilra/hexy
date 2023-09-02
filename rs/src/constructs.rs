use core::marker::Tuple;
use core::ops::*;

pub trait Constructor<T: Tuple + Copy> {
    fn new(para: T) -> Self;
}

pub trait Components<const N: usize>: Copy {
    type Comp: Copy;

    fn from_components(comps: [Self::Comp; N]) -> Self;
    fn components(self) -> [Self::Comp; N];
}

impl Components<1> for f64 {
    type Comp = f64;

    fn from_components(comps: [Self::Comp; 1]) -> Self {
        comps[0]
    }
    fn components(self) -> [Self::Comp; 1] {
        [self]
    }
}

macro_rules! define {
    {
        $self:ident: $n:literal * $ty:ty;
        $name:ident($($field:ident),*);

        $(
            constructor {
                $(($($new_para_name:ident: $new_para_ty:ty),*) => $new_value:expr;)*
            }
        )?

        $(
            accessor {
                $($acc_name:ident: $acc_ty:ty => $acc_value:expr;)*
            }
        )?

        $(
            swizzle {
                $($swizzle_name:ident($($swizzle_field:ident),*) -> $swizzle_ty:ty;)*
            }
        )?

        $(
            op {
                $($op_ty:ty: $({$($op_additional:item)*})? $op_fn:ident($($op_para_name:ident: $op_para_ty:ty),*) -> $op_ret:ty => $op_value:expr;)*
            }
        )?

        $(
            assosciated {
                $($assoc:item)*
            }
        )?
    } => {
        // ($(define!(discard $field -> $ty),)*) expands to a tuple of $ty with $n length

        #[derive(Debug, Clone, Copy, PartialEq)]
        #[repr(C)]
        pub struct $name {
            $(pub $field: $ty),*
        }

        pub fn $name<T: Tuple + Copy>(para: T) -> $name where $name: Constructor<T> { $name::new(para) }

        impl $name {
            $($(
                pub fn $acc_name($self) -> $acc_ty { $acc_value }
            )*)?

            $($(
                pub fn $swizzle_name(self) -> $swizzle_ty { <$swizzle_ty>::new(($(self.$swizzle_field,)*)) }
            )*)?

            $($($assoc)*)?
        }

        impl Components<$n> for $name {
            type Comp = $ty;

            fn from_components(comps: [Self::Comp; $n]) -> Self { Self::new((comps,)) }
            fn components(self) -> [Self::Comp; $n] { [$(self.$field),*] }
        }

        impl Deref for $name {
            type Target = ($(define!(discard $field -> $ty),)*);

            fn deref(&self) -> &Self::Target { unsafe { &*(self as *const Self as *const Self::Target) } }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target { unsafe { &mut*(self as *mut Self as *mut Self::Target) } }
        }

        impl Constructor<($name,)> for $name {
            fn new(($name,): ($name,)) -> Self {
                $name
            }
        }

        impl Constructor<($(define!(discard $field -> $ty),)*)> for $name {
            fn new(($($field,)*): ($(define!(discard $field -> $ty),)*)) -> Self {
                Self {
                    $($field,)*
                }
            }
        }

        impl Constructor<([$ty; $n],)> for $name {
            fn new(([$($field,)*],): ([$ty; $n],)) -> Self {
                Self {
                    $($field,)*
                }
            }
        }

        $($(
            #[allow(unused)]
            impl Constructor<($($new_para_ty,)*)> for $name {
                fn new(($($new_para_name,)*): ($($new_para_ty,)*)) -> Self { $new_value }
            }
        )*)?

        $($(
            #[allow(unused)]
            impl $op_ty for $name {
                $($($op_additional)*)?

                fn $op_fn($($op_para_name: $op_para_ty),*) -> $op_ret { $op_value }
            }
        )*)?
    };

    (discard $discard:tt -> $keep:tt) => { $keep };
    (len [$($tt:tt),*]) => { [$(define!(discard $tt -> ())),*].len() };
}

macro_rules! num_impl {
    ($n:literal * $ty:ty: $name:ident($($field:ident),*)) => {
        num_impl! {
            impl($name);

            Add: {
                type Output = $name;
            } add(self: $name, rhs: $name) -> $name => $name(($(self.$field + rhs.$field,)*));
            AddAssign: add_assign(self: &mut $name, rhs: $name) -> () => {
                $(self.$field += rhs.$field;)*;
            };

            Sub: {
                type Output = $name;
            } sub(self: $name, rhs: $name) -> $name => $name(($(self.$field - rhs.$field,)*));
            SubAssign: sub_assign(self: &mut $name, rhs: $name) -> () => {
                $(self.$field -= rhs.$field;)*;
            };

            Mul: {
                type Output = $name;
            } mul(self: $name, rhs: $name) -> $name => $name(($(self.$field * rhs.$field,)*));
            Mul<f64>: {
                type Output = $name;
            } mul(self: $name, rhs: f64) -> $name => $name(($(self.$field * rhs,)*));
            MulAssign: mul_assign(self: &mut $name, rhs: $name) -> () => {
                $(self.$field *= rhs.$field;)*;
            };
            MulAssign<f64>: mul_assign(self: &mut $name, rhs: f64) -> () => {
                $(self.$field *= rhs;)*;
            };

            Div: {
                type Output = $name;
            } div(self: $name, rhs: $name) -> $name => $name(($(self.$field / rhs.$field,)*));
            Div<f64>: {
                type Output = $name;
            } div(self: $name, rhs: f64) -> $name => $name(($(self.$field / rhs,)*));
            DivAssign: div_assign(self: &mut $name, rhs: $name) -> () => {
                $(self.$field /= rhs.$field;)*;
            };
            DivAssign<f64>: div_assign(self: &mut $name, rhs: f64) -> () => {
                $(self.$field /= rhs;)*;
            };
        }
    };
    {
        impl($name:ident);

        $($op_ty:ty: $({$($op_additional:item)*})? $op_fn:ident($($op_para_name:ident: $op_para_ty:ty),*) -> $op_ret:ty => $op_value:expr;)*
    } => {
        $(
            #[allow(unused)]
            impl $op_ty for $name {
                $($($op_additional)*)?

                fn $op_fn($($op_para_name: $op_para_ty),*) -> $op_ret { $op_value }
            }
        )*
    }
}

define! {
    self: 2 * f64;
    vec2(x, y);

    constructor {
        (single: f64) => vec2((single, single));
    }

    swizzle {
        xx(x, x) -> vec2;
        xy(x, y) -> vec2;
        yx(y, x) -> vec2;
        yy(y, y) -> vec2;

        xxx(x, x, x) -> vec3;
        xxy(x, x, y) -> vec3;
        xyx(x, y, x) -> vec3;
        xyy(x, y, y) -> vec3;
        yxx(y, x, x) -> vec3;
        yxy(y, x, y) -> vec3;
        yyx(y, y, x) -> vec3;
        yyy(y, y, y) -> vec3;

        xxxx(x, x, x, x) -> vec4;
        xxxy(x, x, x, y) -> vec4;
        xxyx(x, x, y, x) -> vec4;
        xxyy(x, x, y, y) -> vec4;
        xyxx(x, y, x, x) -> vec4;
        xyxy(x, y, x, y) -> vec4;
        xyyx(x, y, y, x) -> vec4;
        xyyy(x, y, y, y) -> vec4;
        yxxx(y, x, x, x) -> vec4;
        yxxy(y, x, x, y) -> vec4;
        yxyx(y, x, y, x) -> vec4;
        yxyy(y, x, y, y) -> vec4;
        yyxx(y, y, x, x) -> vec4;
        yyxy(y, y, x, y) -> vec4;
        yyyx(y, y, y, x) -> vec4;
        yyyy(y, y, y, y) -> vec4;
    }
}

num_impl!(2 * f64: vec2(x, y));

define! {
    self: 3 * f64;
    vec3(x, y, z);

    constructor {
        (single: f64) => vec3((single, single, single));
        (vec2: vec2, z: f64) => vec3((vec2.x, vec2.y, z));
    }

    swizzle {
        xx(x, x) -> vec2;
        xy(x, y) -> vec2;
        xz(x, z) -> vec2;
        yx(y, x) -> vec2;
        yy(y, y) -> vec2;
        yz(y, z) -> vec2;
        zx(z, x) -> vec2;
        zy(z, y) -> vec2;
        zz(z, z) -> vec2;

        xxx(x, x, x) -> vec3;
        xxy(x, x, y) -> vec3;
        xxz(x, x, z) -> vec3;
        xyx(x, y, x) -> vec3;
        xyy(x, y, y) -> vec3;
        xyz(x, y, z) -> vec3;
        xzx(x, z, x) -> vec3;
        xzy(x, z, y) -> vec3;
        xzz(x, z, z) -> vec3;
        yxx(y, x, x) -> vec3;
        yxy(y, x, y) -> vec3;
        yxz(y, x, z) -> vec3;
        yyx(y, y, x) -> vec3;
        yyy(y, y, y) -> vec3;
        yyz(y, y, z) -> vec3;
        yzx(y, z, x) -> vec3;
        yzy(y, z, y) -> vec3;
        yzz(y, z, z) -> vec3;
        zxx(z, x, x) -> vec3;
        zxy(z, x, y) -> vec3;
        zxz(z, x, z) -> vec3;
        zyx(z, y, x) -> vec3;
        zyy(z, y, y) -> vec3;
        zyz(z, y, z) -> vec3;
        zzx(z, z, x) -> vec3;
        zzy(z, z, y) -> vec3;
        zzz(z, z, z) -> vec3;

        xxxx(x, x, x, x) -> vec4;
        xxxy(x, x, x, y) -> vec4;
        xxxz(x, x, x, z) -> vec4;
        xxyx(x, x, y, x) -> vec4;
        xxyy(x, x, y, y) -> vec4;
        xxyz(x, x, y, z) -> vec4;
        xxzx(x, x, z, x) -> vec4;
        xxzy(x, x, z, y) -> vec4;
        xxzz(x, x, z, z) -> vec4;
        xyxx(x, y, x, x) -> vec4;
        xyxy(x, y, x, y) -> vec4;
        xyxz(x, y, x, z) -> vec4;
        xyyx(x, y, y, x) -> vec4;
        xyyy(x, y, y, y) -> vec4;
        xyyz(x, y, y, z) -> vec4;
        xyzx(x, y, z, x) -> vec4;
        xyzy(x, y, z, y) -> vec4;
        xyzz(x, y, z, z) -> vec4;
        xzxx(x, z, x, x) -> vec4;
        xzxy(x, z, x, y) -> vec4;
        xzxz(x, z, x, z) -> vec4;
        xzyx(x, z, y, x) -> vec4;
        xzyy(x, z, y, y) -> vec4;
        xzyz(x, z, y, z) -> vec4;
        xzzx(x, z, z, x) -> vec4;
        xzzy(x, z, z, y) -> vec4;
        xzzz(x, z, z, z) -> vec4;
        yxxx(y, x, x, x) -> vec4;
        yxxy(y, x, x, y) -> vec4;
        yxxz(y, x, x, z) -> vec4;
        yxyx(y, x, y, x) -> vec4;
        yxyy(y, x, y, y) -> vec4;
        yxyz(y, x, y, z) -> vec4;
        yxzx(y, x, z, x) -> vec4;
        yxzy(y, x, z, y) -> vec4;
        yxzz(y, x, z, z) -> vec4;
        yyxx(y, y, x, x) -> vec4;
        yyxy(y, y, x, y) -> vec4;
        yyxz(y, y, x, z) -> vec4;
        yyyx(y, y, y, x) -> vec4;
        yyyy(y, y, y, y) -> vec4;
        yyyz(y, y, y, z) -> vec4;
        yyzx(y, y, z, x) -> vec4;
        yyzy(y, y, z, y) -> vec4;
        yyzz(y, y, z, z) -> vec4;
        yzxx(y, z, x, x) -> vec4;
        yzxy(y, z, x, y) -> vec4;
        yzxz(y, z, x, z) -> vec4;
        yzyx(y, z, y, x) -> vec4;
        yzyy(y, z, y, y) -> vec4;
        yzyz(y, z, y, z) -> vec4;
        yzzx(y, z, z, x) -> vec4;
        yzzy(y, z, z, y) -> vec4;
        yzzz(y, z, z, z) -> vec4;
        zxxx(z, x, x, x) -> vec4;
        zxxy(z, x, x, y) -> vec4;
        zxxz(z, x, x, z) -> vec4;
        zxyx(z, x, y, x) -> vec4;
        zxyy(z, x, y, y) -> vec4;
        zxyz(z, x, y, z) -> vec4;
        zxzx(z, x, z, x) -> vec4;
        zxzy(z, x, z, y) -> vec4;
        zxzz(z, x, z, z) -> vec4;
        zyxx(z, y, x, x) -> vec4;
        zyxy(z, y, x, y) -> vec4;
        zyxz(z, y, x, z) -> vec4;
        zyyx(z, y, y, x) -> vec4;
        zyyy(z, y, y, y) -> vec4;
        zyyz(z, y, y, z) -> vec4;
        zyzx(z, y, z, x) -> vec4;
        zyzy(z, y, z, y) -> vec4;
        zyzz(z, y, z, z) -> vec4;
        zzxx(z, z, x, x) -> vec4;
        zzxy(z, z, x, y) -> vec4;
        zzxz(z, z, x, z) -> vec4;
        zzyx(z, z, y, x) -> vec4;
        zzyy(z, z, y, y) -> vec4;
        zzyz(z, z, y, z) -> vec4;
        zzzx(z, z, z, x) -> vec4;
        zzzy(z, z, z, y) -> vec4;
        zzzz(z, z, z, z) -> vec4;
    }
}

num_impl!(3 * f64: vec3(x, y, z));

define! {
    self: 4 * f64;
    vec4(x, y, z, w);

    constructor {
        (single: f64) => vec4((single, single, single, single));
        (vec2: vec2, z: f64, w: f64) => vec4((vec2.x, vec2.y, z, w));
        (vec3: vec3, w: f64) => vec4((vec3.x, vec3.y, vec3.z, w));
    }

    swizzle {
        xx(x, x) -> vec2;
        xy(x, y) -> vec2;
        xz(x, z) -> vec2;
        xw(x, w) -> vec2;
        yx(y, x) -> vec2;
        yy(y, y) -> vec2;
        yz(y, z) -> vec2;
        yw(y, w) -> vec2;
        zx(z, x) -> vec2;
        zy(z, y) -> vec2;
        zz(z, z) -> vec2;
        zw(z, w) -> vec2;
        wx(w, x) -> vec2;
        wy(w, y) -> vec2;
        wz(w, z) -> vec2;
        ww(w, w) -> vec2;

        xxx(x, x, x) -> vec3;
        xxy(x, x, y) -> vec3;
        xxz(x, x, z) -> vec3;
        xxw(x, x, w) -> vec3;
        xyx(x, y, x) -> vec3;
        xyy(x, y, y) -> vec3;
        xyz(x, y, z) -> vec3;
        xyw(x, y, w) -> vec3;
        xzx(x, z, x) -> vec3;
        xzy(x, z, y) -> vec3;
        xzz(x, z, z) -> vec3;
        xzw(x, z, w) -> vec3;
        xwx(x, w, x) -> vec3;
        xwy(x, w, y) -> vec3;
        xwz(x, w, z) -> vec3;
        xww(x, w, w) -> vec3;
        yxx(y, x, x) -> vec3;
        yxy(y, x, y) -> vec3;
        yxz(y, x, z) -> vec3;
        yxw(y, x, w) -> vec3;
        yyx(y, y, x) -> vec3;
        yyy(y, y, y) -> vec3;
        yyz(y, y, z) -> vec3;
        yyw(y, y, w) -> vec3;
        yzx(y, z, x) -> vec3;
        yzy(y, z, y) -> vec3;
        yzz(y, z, z) -> vec3;
        yzw(y, z, w) -> vec3;
        ywx(y, w, x) -> vec3;
        ywy(y, w, y) -> vec3;
        ywz(y, w, z) -> vec3;
        yww(y, w, w) -> vec3;
        zxx(z, x, x) -> vec3;
        zxy(z, x, y) -> vec3;
        zxz(z, x, z) -> vec3;
        zxw(z, x, w) -> vec3;
        zyx(z, y, x) -> vec3;
        zyy(z, y, y) -> vec3;
        zyz(z, y, z) -> vec3;
        zyw(z, y, w) -> vec3;
        zzx(z, z, x) -> vec3;
        zzy(z, z, y) -> vec3;
        zzz(z, z, z) -> vec3;
        zzw(z, z, w) -> vec3;
        zwx(z, w, x) -> vec3;
        zwy(z, w, y) -> vec3;
        zwz(z, w, z) -> vec3;
        zww(z, w, w) -> vec3;
        wxx(w, x, x) -> vec3;
        wxy(w, x, y) -> vec3;
        wxz(w, x, z) -> vec3;
        wxw(w, x, w) -> vec3;
        wyx(w, y, x) -> vec3;
        wyy(w, y, y) -> vec3;
        wyz(w, y, z) -> vec3;
        wyw(w, y, w) -> vec3;
        wzx(w, z, x) -> vec3;
        wzy(w, z, y) -> vec3;
        wzz(w, z, z) -> vec3;
        wzw(w, z, w) -> vec3;
        wwx(w, w, x) -> vec3;
        wwy(w, w, y) -> vec3;
        wwz(w, w, z) -> vec3;
        www(w, w, w) -> vec3;

        xxxx(x, x, x, x) -> vec4;
        xxxy(x, x, x, y) -> vec4;
        xxxz(x, x, x, z) -> vec4;
        xxxw(x, x, x, w) -> vec4;
        xxyx(x, x, y, x) -> vec4;
        xxyy(x, x, y, y) -> vec4;
        xxyz(x, x, y, z) -> vec4;
        xxyw(x, x, y, w) -> vec4;
        xxzx(x, x, z, x) -> vec4;
        xxzy(x, x, z, y) -> vec4;
        xxzz(x, x, z, z) -> vec4;
        xxzw(x, x, z, w) -> vec4;
        xxwx(x, x, w, x) -> vec4;
        xxwy(x, x, w, y) -> vec4;
        xxwz(x, x, w, z) -> vec4;
        xxww(x, x, w, w) -> vec4;
        xyxx(x, y, x, x) -> vec4;
        xyxy(x, y, x, y) -> vec4;
        xyxz(x, y, x, z) -> vec4;
        xyxw(x, y, x, w) -> vec4;
        xyyx(x, y, y, x) -> vec4;
        xyyy(x, y, y, y) -> vec4;
        xyyz(x, y, y, z) -> vec4;
        xyyw(x, y, y, w) -> vec4;
        xyzx(x, y, z, x) -> vec4;
        xyzy(x, y, z, y) -> vec4;
        xyzz(x, y, z, z) -> vec4;
        xyzw(x, y, z, w) -> vec4;
        xywx(x, y, w, x) -> vec4;
        xywy(x, y, w, y) -> vec4;
        xywz(x, y, w, z) -> vec4;
        xyww(x, y, w, w) -> vec4;
        xzxx(x, z, x, x) -> vec4;
        xzxy(x, z, x, y) -> vec4;
        xzxz(x, z, x, z) -> vec4;
        xzxw(x, z, x, w) -> vec4;
        xzyx(x, z, y, x) -> vec4;
        xzyy(x, z, y, y) -> vec4;
        xzyz(x, z, y, z) -> vec4;
        xzyw(x, z, y, w) -> vec4;
        xzzx(x, z, z, x) -> vec4;
        xzzy(x, z, z, y) -> vec4;
        xzzz(x, z, z, z) -> vec4;
        xzzw(x, z, z, w) -> vec4;
        xzwx(x, z, w, x) -> vec4;
        xzwy(x, z, w, y) -> vec4;
        xzwz(x, z, w, z) -> vec4;
        xzww(x, z, w, w) -> vec4;
        xwxx(x, w, x, x) -> vec4;
        xwxy(x, w, x, y) -> vec4;
        xwxz(x, w, x, z) -> vec4;
        xwxw(x, w, x, w) -> vec4;
        xwyx(x, w, y, x) -> vec4;
        xwyy(x, w, y, y) -> vec4;
        xwyz(x, w, y, z) -> vec4;
        xwyw(x, w, y, w) -> vec4;
        xwzx(x, w, z, x) -> vec4;
        xwzy(x, w, z, y) -> vec4;
        xwzz(x, w, z, z) -> vec4;
        xwzw(x, w, z, w) -> vec4;
        xwwx(x, w, w, x) -> vec4;
        xwwy(x, w, w, y) -> vec4;
        xwwz(x, w, w, z) -> vec4;
        xwww(x, w, w, w) -> vec4;
        yxxx(y, x, x, x) -> vec4;
        yxxy(y, x, x, y) -> vec4;
        yxxz(y, x, x, z) -> vec4;
        yxxw(y, x, x, w) -> vec4;
        yxyx(y, x, y, x) -> vec4;
        yxyy(y, x, y, y) -> vec4;
        yxyz(y, x, y, z) -> vec4;
        yxyw(y, x, y, w) -> vec4;
        yxzx(y, x, z, x) -> vec4;
        yxzy(y, x, z, y) -> vec4;
        yxzz(y, x, z, z) -> vec4;
        yxzw(y, x, z, w) -> vec4;
        yxwx(y, x, w, x) -> vec4;
        yxwy(y, x, w, y) -> vec4;
        yxwz(y, x, w, z) -> vec4;
        yxww(y, x, w, w) -> vec4;
        yyxx(y, y, x, x) -> vec4;
        yyxy(y, y, x, y) -> vec4;
        yyxz(y, y, x, z) -> vec4;
        yyxw(y, y, x, w) -> vec4;
        yyyx(y, y, y, x) -> vec4;
        yyyy(y, y, y, y) -> vec4;
        yyyz(y, y, y, z) -> vec4;
        yyyw(y, y, y, w) -> vec4;
        yyzx(y, y, z, x) -> vec4;
        yyzy(y, y, z, y) -> vec4;
        yyzz(y, y, z, z) -> vec4;
        yyzw(y, y, z, w) -> vec4;
        yywx(y, y, w, x) -> vec4;
        yywy(y, y, w, y) -> vec4;
        yywz(y, y, w, z) -> vec4;
        yyww(y, y, w, w) -> vec4;
        yzxx(y, z, x, x) -> vec4;
        yzxy(y, z, x, y) -> vec4;
        yzxz(y, z, x, z) -> vec4;
        yzxw(y, z, x, w) -> vec4;
        yzyx(y, z, y, x) -> vec4;
        yzyy(y, z, y, y) -> vec4;
        yzyz(y, z, y, z) -> vec4;
        yzyw(y, z, y, w) -> vec4;
        yzzx(y, z, z, x) -> vec4;
        yzzy(y, z, z, y) -> vec4;
        yzzz(y, z, z, z) -> vec4;
        yzzw(y, z, z, w) -> vec4;
        yzwx(y, z, w, x) -> vec4;
        yzwy(y, z, w, y) -> vec4;
        yzwz(y, z, w, z) -> vec4;
        yzww(y, z, w, w) -> vec4;
        ywxx(y, w, x, x) -> vec4;
        ywxy(y, w, x, y) -> vec4;
        ywxz(y, w, x, z) -> vec4;
        ywxw(y, w, x, w) -> vec4;
        ywyx(y, w, y, x) -> vec4;
        ywyy(y, w, y, y) -> vec4;
        ywyz(y, w, y, z) -> vec4;
        ywyw(y, w, y, w) -> vec4;
        ywzx(y, w, z, x) -> vec4;
        ywzy(y, w, z, y) -> vec4;
        ywzz(y, w, z, z) -> vec4;
        ywzw(y, w, z, w) -> vec4;
        ywwx(y, w, w, x) -> vec4;
        ywwy(y, w, w, y) -> vec4;
        ywwz(y, w, w, z) -> vec4;
        ywww(y, w, w, w) -> vec4;
        zxxx(z, x, x, x) -> vec4;
        zxxy(z, x, x, y) -> vec4;
        zxxz(z, x, x, z) -> vec4;
        zxxw(z, x, x, w) -> vec4;
        zxyx(z, x, y, x) -> vec4;
        zxyy(z, x, y, y) -> vec4;
        zxyz(z, x, y, z) -> vec4;
        zxyw(z, x, y, w) -> vec4;
        zxzx(z, x, z, x) -> vec4;
        zxzy(z, x, z, y) -> vec4;
        zxzz(z, x, z, z) -> vec4;
        zxzw(z, x, z, w) -> vec4;
        zxwx(z, x, w, x) -> vec4;
        zxwy(z, x, w, y) -> vec4;
        zxwz(z, x, w, z) -> vec4;
        zxww(z, x, w, w) -> vec4;
        zyxx(z, y, x, x) -> vec4;
        zyxy(z, y, x, y) -> vec4;
        zyxz(z, y, x, z) -> vec4;
        zyxw(z, y, x, w) -> vec4;
        zyyx(z, y, y, x) -> vec4;
        zyyy(z, y, y, y) -> vec4;
        zyyz(z, y, y, z) -> vec4;
        zyyw(z, y, y, w) -> vec4;
        zyzx(z, y, z, x) -> vec4;
        zyzy(z, y, z, y) -> vec4;
        zyzz(z, y, z, z) -> vec4;
        zyzw(z, y, z, w) -> vec4;
        zywx(z, y, w, x) -> vec4;
        zywy(z, y, w, y) -> vec4;
        zywz(z, y, w, z) -> vec4;
        zyww(z, y, w, w) -> vec4;
        zzxx(z, z, x, x) -> vec4;
        zzxy(z, z, x, y) -> vec4;
        zzxz(z, z, x, z) -> vec4;
        zzxw(z, z, x, w) -> vec4;
        zzyx(z, z, y, x) -> vec4;
        zzyy(z, z, y, y) -> vec4;
        zzyz(z, z, y, z) -> vec4;
        zzyw(z, z, y, w) -> vec4;
        zzzx(z, z, z, x) -> vec4;
        zzzy(z, z, z, y) -> vec4;
        zzzz(z, z, z, z) -> vec4;
        zzzw(z, z, z, w) -> vec4;
        zzwx(z, z, w, x) -> vec4;
        zzwy(z, z, w, y) -> vec4;
        zzwz(z, z, w, z) -> vec4;
        zzww(z, z, w, w) -> vec4;
        zwxx(z, w, x, x) -> vec4;
        zwxy(z, w, x, y) -> vec4;
        zwxz(z, w, x, z) -> vec4;
        zwxw(z, w, x, w) -> vec4;
        zwyx(z, w, y, x) -> vec4;
        zwyy(z, w, y, y) -> vec4;
        zwyz(z, w, y, z) -> vec4;
        zwyw(z, w, y, w) -> vec4;
        zwzx(z, w, z, x) -> vec4;
        zwzy(z, w, z, y) -> vec4;
        zwzz(z, w, z, z) -> vec4;
        zwzw(z, w, z, w) -> vec4;
        zwwx(z, w, w, x) -> vec4;
        zwwy(z, w, w, y) -> vec4;
        zwwz(z, w, w, z) -> vec4;
        zwww(z, w, w, w) -> vec4;
        wxxx(w, x, x, x) -> vec4;
        wxxy(w, x, x, y) -> vec4;
        wxxz(w, x, x, z) -> vec4;
        wxxw(w, x, x, w) -> vec4;
        wxyx(w, x, y, x) -> vec4;
        wxyy(w, x, y, y) -> vec4;
        wxyz(w, x, y, z) -> vec4;
        wxyw(w, x, y, w) -> vec4;
        wxzx(w, x, z, x) -> vec4;
        wxzy(w, x, z, y) -> vec4;
        wxzz(w, x, z, z) -> vec4;
        wxzw(w, x, z, w) -> vec4;
        wxwx(w, x, w, x) -> vec4;
        wxwy(w, x, w, y) -> vec4;
        wxwz(w, x, w, z) -> vec4;
        wxww(w, x, w, w) -> vec4;
        wyxx(w, y, x, x) -> vec4;
        wyxy(w, y, x, y) -> vec4;
        wyxz(w, y, x, z) -> vec4;
        wyxw(w, y, x, w) -> vec4;
        wyyx(w, y, y, x) -> vec4;
        wyyy(w, y, y, y) -> vec4;
        wyyz(w, y, y, z) -> vec4;
        wyyw(w, y, y, w) -> vec4;
        wyzx(w, y, z, x) -> vec4;
        wyzy(w, y, z, y) -> vec4;
        wyzz(w, y, z, z) -> vec4;
        wyzw(w, y, z, w) -> vec4;
        wywx(w, y, w, x) -> vec4;
        wywy(w, y, w, y) -> vec4;
        wywz(w, y, w, z) -> vec4;
        wyww(w, y, w, w) -> vec4;
        wzxx(w, z, x, x) -> vec4;
        wzxy(w, z, x, y) -> vec4;
        wzxz(w, z, x, z) -> vec4;
        wzxw(w, z, x, w) -> vec4;
        wzyx(w, z, y, x) -> vec4;
        wzyy(w, z, y, y) -> vec4;
        wzyz(w, z, y, z) -> vec4;
        wzyw(w, z, y, w) -> vec4;
        wzzx(w, z, z, x) -> vec4;
        wzzy(w, z, z, y) -> vec4;
        wzzz(w, z, z, z) -> vec4;
        wzzw(w, z, z, w) -> vec4;
        wzwx(w, z, w, x) -> vec4;
        wzwy(w, z, w, y) -> vec4;
        wzwz(w, z, w, z) -> vec4;
        wzww(w, z, w, w) -> vec4;
        wwxx(w, w, x, x) -> vec4;
        wwxy(w, w, x, y) -> vec4;
        wwxz(w, w, x, z) -> vec4;
        wwxw(w, w, x, w) -> vec4;
        wwyx(w, w, y, x) -> vec4;
        wwyy(w, w, y, y) -> vec4;
        wwyz(w, w, y, z) -> vec4;
        wwyw(w, w, y, w) -> vec4;
        wwzx(w, w, z, x) -> vec4;
        wwzy(w, w, z, y) -> vec4;
        wwzz(w, w, z, z) -> vec4;
        wwzw(w, w, z, w) -> vec4;
        wwwx(w, w, w, x) -> vec4;
        wwwy(w, w, w, y) -> vec4;
        wwwz(w, w, w, z) -> vec4;
        wwww(w, w, w, w) -> vec4;
    }
}

num_impl!(4 * f64: vec4(x, y, z, w));

define! {
    self: 2 * [f64; 2];
    mat2(mx1, mx2);

    constructor {
        (m11: f64, m21: f64, m12: f64, m22: f64) => mat2(([m11, m21], [m12, m22]));
    }

    accessor {
        m11: f64 => self.mx1[0];
        m21: f64 => self.mx1[1];
        m12: f64 => self.mx2[0];
        m22: f64 => self.mx2[1];
    }

    op {
        Mul<vec2>: {
            type Output = vec2;
        } mul(self: mat2, vec: vec2) -> vec2 => vec2((vec.x * self.m11() + vec.y * self.m12(), vec.x * self.m21() + vec.y * self.m22()));
    }
}
