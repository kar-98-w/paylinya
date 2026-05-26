#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, Address};

    fn setup() -> (Env, Address, Address) {
        let env = Env::default();
        let student = Address::generate(&env);
        let vendor = Address::generate(&env);
        (env, student, vendor)
    }

    // Test 1: Happy path payment
    #[test]
    fn test_payment_success() {
        let (env, student, _) = setup();

        PayLinyaContract::pay(env.clone(), student.clone(), 100);

        assert_eq!(
            PayLinyaContract::get_payment(env, student),
            100
        );
    }

    // Test 2: Unauthorized behavior simulation (no payment)
    #[test]
    fn test_no_payment() {
        let (env, student, _) = setup();

        assert_eq!(
            PayLinyaContract::get_payment(env, student),
            0
        );
    }

    // Test 3: Verify payment success
    #[test]
    fn test_verify_payment_true() {
        let (env, student, _) = setup();

        PayLinyaContract::pay(env.clone(), student.clone(), 50);

        assert_eq!(
            PayLinyaContract::verify_payment(env, student),
            true
        );
    }

    // Test 4: Verify payment false
    #[test]
    fn test_verify_payment_false() {
        let (env, student, _) = setup();

        assert_eq!(
            PayLinyaContract::verify_payment(env, student),
            false
        );
    }

    // Test 5: State persistence check
    #[test]
    fn test_state_persistence() {
        let (env, student, _) = setup();

        PayLinyaContract::pay(env.clone(), student.clone(), 200);

        let stored = PayLinyaContract::get_payment(env.clone(), student.clone());

        assert_eq!(stored, 200);
    }
}