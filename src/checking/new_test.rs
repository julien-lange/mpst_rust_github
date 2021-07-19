use regex::Regex;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;

#[macro_export]
macro_rules! checker_concat {
    (
        $( $sessiontype: ty , )+
        $(
            {
                $choice: ty,
                $(
                    $branches: ident
                ),+ $(,)?
            }
        ),+ $(,)?
    ) => {

        fn type_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        let mut sessions = Vec::new();
        let mut tail_sessions = Vec::new();

        $(
            sessions.push(String::from(std::any::type_name::<$sessiontype>()));
            tail_sessions.push(<$sessiontype as mpstthree::binary::struct_trait::Session>::tail_str());
        )+

        mpst_seq::checking!(
            $(
                {
                    $choice: ty,
                    $(
                        $branches: ident,
                    )+
                }
            )+
        );

        println!("{:?}", mpstthree::checking::new_test::checker(sessions, tail_sessions, branches_receivers));
    };
}

#[doc(hidden)]
fn clean_sesion(session: String) -> Result<Vec<String>, Box<dyn Error>> {
    // The regex expression
    let main_re = Regex::new(r"([^<,>\s]+)::([^<,>\s]+)").unwrap();
    let mut temp = String::from(&session);

    // Replace with regex expression -> term1::term2::term3 by term3
    for caps in main_re.captures_iter(&session) {
        temp = temp.replace(&caps[0], &caps[caps.len() - 1]);
    }

    // Remove whitespaces
    temp.retain(|c| !c.is_whitespace());

    // Get each field of the MeshedChannels
    let mut full_block = get_blocks(temp)?;

    // Get the name of the role
    let name = &full_block[full_block.len() - 1]
        .split(['<', '>'].as_ref())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect::<Vec<_>>()[0];

    full_block.push(String::from(name));

    Ok(full_block)
}

/// Clean the sessions received and returns a Hashmap of the cleaned sessions and their respective role.
///
/// Remove the unnecessary terms before each :: (such as mpstthree in mpstthree::Session),
/// and link each new String with its respective role.
#[doc(hidden)]
fn clean_sessions(sessions: Vec<String>) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    // The hasher of the HashMap
    let state_branches_receivers = RandomState::new();

    // The result
    let mut all_sessions: HashMap<String, Vec<String>> =
        HashMap::with_hasher(state_branches_receivers);

    for session in sessions {
        let full_block = clean_sesion(session)?;

        let name = String::from(&full_block[full_block.len() - 1]);

        // Insert the vec of fields (minus the name's role) linked to the name of the role
        all_sessions.insert(name, full_block[..(full_block.len() - 2)].to_vec());
    }

    Ok(all_sessions)
}

// Get the roles from the tail_sessions (the ones from the method tail_str for MeshedChannels).
#[doc(hidden)]
fn roles(tail_sessions: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut roles = Vec::new();
    for tail_session in tail_sessions {
        // Split according to '\n'
        let full_vec: Vec<&str> = tail_session
            .split('\n')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        // Split according to both '<' and '>', and append to result
        roles.append(
            &mut full_vec[full_vec.len() - 2]
                .split(['<', '>'].as_ref())
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect::<Vec<_>>(),
        );

        // Split and push the name of the role of the MeshecChannels
        roles.push(String::from(
            full_vec[full_vec.len() - 1].split('<').collect::<Vec<_>>()[0],
        ));
    }

    // Remove RoleBroadcast and RoleEnd
    roles = roles
        .iter()
        .filter(|s| *s != "RoleBroadcast" && *s != "RoleEnd")
        .map(String::from)
        .collect();

    // Sort
    roles.sort();

    // Remove duplicates
    roles.dedup();

    Ok(roles)
}

/// Separate the different _fields_ of a stringified MeshedChannels.
#[doc(hidden)]
fn get_blocks(full_block: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result = Vec::new();
    let mut temp = String::from("");
    let mut index = -1;

    for i in full_block.chars() {
        if i == '&' {
        } else if i == '>' && index == 0 {
            result.push(format!("{}{}", temp, i));
            temp = String::from("");
        } else if i == '<' && index >= 0 {
            temp = format!("{}{}", temp, i);
            index += 1;
        } else if i == '>' && index >= 0 {
            temp = format!("{}{}", temp, i);
            index -= 1;
        } else if i == ',' && index == 0 {
            result.push(temp);
            temp = String::from("");
        } else if index >= 0 {
            temp = format!("{}{}", temp, i);
        } else if i == '<' {
            index += 1;
        } else if i == '>' {
            index -= 1;
        }
    }

    if !temp.is_empty() {
        result.push(temp);
    }

    Ok(result)
}

pub fn checker(
    sessions: Vec<String>,
    tail_sessions: Vec<String>,
    branches_receivers: HashMap<String, std::collections::HashMap<String, String>>,
) -> Result<Vec<String>, Box<dyn Error>> {
    println!("sessions: {:?}", &sessions);

    let clean_sessions = clean_sessions(sessions)?;
    let roles = roles(tail_sessions)?;

    println!("clean_sessions: {:?}", &clean_sessions);
    println!("roles: {:?}", &roles);
    println!("branches_receivers: {:?}", &branches_receivers);

    Ok(vec![String::from("")])
}
