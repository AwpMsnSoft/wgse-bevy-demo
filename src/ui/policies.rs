// /// This macro is used to check if the given struct contains members required by the `#policy_name` policy
// ///     - if the struct contains all the required members, no code is generated
// ///     - if the struct does not contain all the required members, a `panic!` will happened in compile time
// ///
// /// * Parameters:
// ///     - $policy: the name of the policy, e.g. `FocusPolicy`
// ///     - $policy_types: the string of policy member types, devided by ` `
// ///     - $target: the name of the struct to be checked, e.g. `ImageBundle`
// ///     - $target_types: the string of the struct member types, devided by ` `
// ///
// /// * Returns:
// ///     - cause the panic! if the struct does not contain all the required members
// ///     - otherwise, nothing will happen in run time
// #[macro_export]
// macro_rules! has_policy_members {
//     ($policy: expr, $policy_types: expr, $target: expr, $target_types: expr) => {{
//         let policy_member_types = $policy_types.split(' ').collect::<Vec<&str>>();
//         let target_member_types = $target_types.split(' ').collect::<Vec<&str>>();
//         for policy_member_type in policy_member_types {
//             if !target_member_types.contains(&policy_member_type) {
//                 println!("The struct {} does not contain all the members required by the `{}` policy: member {} is missing.",
//                         $target, $policy, policy_member_type);
//             }
//         }
//     }};
// }

// #[policy]
// pub struct Test {
//     pub _1: String,
//     pub _2: UiImageSet,
//     pub _3: UiWidgetId,
// }