use pbc_zk::*;

/// Perform a zk computation on secret-shared data campaign the secret variables.
///
/// ### Returns:
///
/// The selected campaign of the secret variables.
pub fn zk_compute() -> Sbi32 {
    // Initialize state
    let mut selected_campaign: Sbi32 = sbi32_from(sbi32_metadata(1));
    let mut selected_campaign_coeff: Sbi32 = sbi32_from(0);
    let mut minimum_delta: Sbi32 = sbi32_from(0)

    // Determine better campaign
    for variable_id in 1..(num_secret_variables() + 1) {
        if abs(sbi32_input(variable_id) - selected_campaign_coeff) > minimum_delta {
            selected_campaign_coeff = sbi32_input(variable_id);
            selected_campaign = sbi32_from(sbi32_metadata(variable_id));
        }
    }
    selected_campaign
}
