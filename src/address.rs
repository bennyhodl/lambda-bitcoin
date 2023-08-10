use bitcoin::address::NetworkUnchecked;
use bitcoin::Address;

pub fn is_address_valid(address: &str) -> bool {
    if let Ok(is_valid) = address.parse::<Address<NetworkUnchecked>>() {
        is_valid.is_valid_for_network(bitcoin::Network::Bitcoin)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_valid() {
        let result = is_address_valid("1FWQiwK27EnGXb6BiBMRLJvunJQZZPMcGd");

        assert_eq!(result, true)
    }
}
