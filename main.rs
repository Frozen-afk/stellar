use std::io::{self, Write};
use std::process::Command;
use std::str;

// --- PROTOTYPE: FAKE SERVICES & DATABASES ---

/// PROTOTYPE: Fakes calling a crypto price API.
/// REAL APP: Would use reqwest and serde to call a real API.
fn get_live_usdc_to_inr_rate() -> f64 {
    println!("\n[API] Fetching live USDC/INR exchange rate...");
    // Return a stable, hardcoded rate for the prototype
    83.50
}

/// PROTOTYPE: Fakes the "off-ramp" payment to a bank account.
/// REAL APP: Would call a payment gateway API (like Razorpay, Cashfree).
fn simulate_upi_payout(upi_id: &str, inr_amount: f64) -> bool {
    println!(
        "\n[Bank] Initiating payout of ₹{:.2} to {}...",
        inr_amount, upi_id
    );
    // Simulate a 1.5 second API call
    std::thread::sleep(std::time::Duration::from_millis(1500));
    println!("[Bank] ✅ Success! Payment sent. Check your account.");
    true
}

// --- PROTOTYPE: REAL CLI WRAPPERS ---

/// Uses the stellar-cli to check an account's balance.
/// This function *actually executes* a shell command.
fn get_usdc_balance(wallet_address: &str) -> Result<f64, String> {
    println!(
        "[CLI] Checking balance for wallet {} on testnet...",
        wallet_address
    );

    let output = Command::new("stellar")
        .args([
            "account",
            "balance",
            wallet_address,
            "--network",
            "testnet",
        ])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = str::from_utf8(&output.stdout).unwrap_or("");
                println!("[CLI] Raw Wallet Output:\n---\n{}\n---", stdout);

                // --- PROTOTYPE SHORTCUT ---
                // In a real app, you would parse the stdout string to find
                // the specific "USDC" balance.
                // For this prototype, we'll just print the output and return
                // a hardcoded value to let the flow continue.
                Ok(1000.00)
            } else {
                let stderr = str::from_utf8(&output.stderr).unwrap_or("Unknown CLI error");
                Err(format!("[CLI] Error: {}", stderr))
            }
        }
        Err(e) => Err(format!("Failed to execute 'stellar' command. Is it in your PATH? Error: {}", e)),
    }
}

/// PROTOTYPE: *Builds* the payment command but *does not run it*.
/// This is the most secure way to prototype.
/// We show the user the *exact* command they would need to run.
fn show_payment_command(amount: f64, anchor_wallet: &str) {
    // NOTE: This command requires the asset issuer. This is a common one for testnet.
    let usdc_asset_code = "USDC:GBBD4S...Q3J5"; 
    
    println!("\n--- SECURITY NOTE ---");
    println!("To complete the payment, you would run the following command.");
    println!("The app would ask you to sign this with your wallet, or you'd run it securely.");
    println!("This prototype *will not* ask for your secret key.");
    println!("\n  stellar tx pay {} {} {} --from YOUR_SECRET_KEY --network testnet",
        amount, usdc_asset_code, anchor_wallet);
    println!("\n[Blockchain] ✅ Success! (Simulated) USDC secured by Gig-Pay.");
}

// --- HELPER FUNCTION ---

/// A simple helper to get formatted user input
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// --- MAIN APPLICATION FLOW ---

fn main() {
    println!("====================================");
    println!("  Welcome to Gig-Pay India Portal  ");
    println!("====================================");

    // 1. "Connect Wallet"
    let wallet_address = get_user_input("Enter your Stellar wallet address (e.g., G...): ");
    if wallet_address.is_empty() {
        println!("Error: Wallet address is required.");
        return;
    }

    // 2. Show Balance (by calling the CLI)
    let balance = match get_usdc_balance(&wallet_address) {
        Ok(balance) => balance,
        Err(e) => {
            println!("{}", e);
            println!("(Continuing with a fake balance of 1000.00 for demo purposes.)");
            1000.00 // Default for demo if CLI fails
        }
    };
    println!(
        "\nWallet query complete! Your (demo) balance: ${:.2} USDC",
        balance
    );

    // 3. Get Withdrawal Amount
    let amount_str = get_user_input("\nHow much USDC do you want to withdraw? (e.g., 1000): ");
    let amount_to_withdraw: f64 = match amount_str.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid amount. Exiting.");
            return;
        }
    };

    // 4. Validate Amount
    if amount_to_withdraw > balance {
        println!(
            "Error: Insufficient funds. You only have ${:.2} USDC.",
            balance
        );
        return;
    }
    if amount_to_withdraw <= 0.0 {
        println!("Error: Please enter a positive amount.");
        return;
    }

    // 5. Get UPI ID
    let upi_id = get_user_input("Enter your UPI ID (e.g., yourname@okbank): ");
    if upi_id.is_empty() {
        println!("Error: UPI ID is required.");
        return;
    }

    // 6. Get Exchange Rate and Calculate Payout
    println!("\nCalculating your payout...");
    let rate = get_live_usdc_to_inr_rate();
    let inr_to_receive = amount_to_withdraw * rate;

    // 7. Show Confirmation
    println!("\n--- Review Transaction ---");
    println!("  You are sending: ${:.2} USDC", amount_to_withdraw);
    println!("  Exchange Rate:   $1.00 = ₹{:.2}", rate);
    println!("--------------------------");
    println!("  You will receive:  ₹{:.2} INR", inr_to_receive);
    println!("  To UPI ID:         {}", upi_id);
    println!("--------------------------");

    let confirm = get_user_input("Type 'yes' to confirm: ");

    // 8. Process Transaction
    if confirm.eq_ignore_ascii_case("yes") {
        println!("\nProcessing your transaction.");
        
        // Step 8a: "Take" the USDC (by showing the CLI command)
        let anchor_wallet_address = "G...ANCHOR_WALLET...123"; // Your company's wallet
        show_payment_command(amount_to_withdraw, anchor_wallet_address);
        
        // Step 8b: Send the INR
        if simulate_upi_payout(&upi_id, inr_to_receive) {
            println!("\n🎉 Payout Complete! Thank you for using Gig-Pay.");
            let new_balance = balance - amount_to_withdraw;
            println!("Your new wallet balance is ${:.2} USDC.", new_balance);
        } else {
            println!("Fatal Error: UPI Payout failed. Contacting support.");
            // (A real app would need to refund the USDC here)
        }
    } else {
        println!("\nTransaction cancelled.");
    }
}