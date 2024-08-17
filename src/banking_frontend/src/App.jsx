import React, { useState, useEffect } from 'react';
import { banking_backend } from "../../declarations/banking_backend"

function App() {
  const [withdrawalAmount, setWithdrawalAmount] = useState(0);
  const [originalAmount, setOriginalAmount] = useState(0);

  useEffect(() => {
    // Fetch current amount from the backend
    const fetchAmount = async () => {
      try {
        const amount = await banking_backend.current_amount();
        setOriginalAmount(amount);
        console.log("Original amount = ", amount);
      } catch (error) {
        console.error("Failed to fetch current amount", error);
      }
    };

    fetchAmount();
  }, []);

  const handleWithdraw = async () => {
    try {
      const remainingAmount = await banking_backend.withdraw(parseInt(withdrawalAmount));
      // Update the state with the remaining amount
      if(remainingAmount == 0){
        alert("you dont have money to withdraw")
      }
      setOriginalAmount(remainingAmount);
    } catch (error) {
      console.error("Failed to withdraw", error);
    }
  };

  return (
    <div className="bg-yellow-400 p-6">
      <h1 className="text-xl font-bold mb-4">Banking App</h1>

      <div className="mb-4">
        <p className="text-lg">Current Balance: ${originalAmount.toFixed(2)}</p>
      </div>

      <div className="mb-4">
        <input
          type="number"
          value={withdrawalAmount}
          onChange={(e) => setWithdrawalAmount(e.target.value)}
          placeholder="Enter amount to withdraw"
          className="p-2 border border-gray-300 rounded"
        />
      </div>

      <button
        onClick={handleWithdraw}
        className="bg-blue-500 text-white p-2 rounded"
      >
        Withdraw
      </button>
    </div>
  );
}

export default App;
