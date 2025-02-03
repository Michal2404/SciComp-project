import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
import matplotlib.ticker as mticker

# Compare IDA* vs BFS
def plot_compare_ida_vs_bfs():
    data_bfs = pd.read_csv(f"csv_files/real_experiments/BFS_time_performance.csv")
    data_ida = pd.read_csv(f"csv_files/real_experiments/IDA_time_performance.csv")

    data_ida = data_ida.rename(columns={"IDA* Moves": "Moves", "IDA* Time (s)": "Time"})
    data_bfs = data_bfs.rename(columns={"BFS Moves": "Moves", "BFS Time (s)": "Time"})

    ida_means = data_ida.groupby("Moves")["Time"].mean().reset_index()
    bfs_means = data_bfs.groupby("Moves")["Time"].mean().reset_index()

    plt.figure(figsize=(10,6))

    plt.plot(ida_means["Moves"], ida_means["Time"], '-o', label='IDA*')
    plt.plot(bfs_means["Moves"], bfs_means["Time"], '-s', label='BFS')

    #plt.yscale('log')

    plt.xlabel('Depth')
    plt.ylabel('Times (s) [Log Scale]')
    plt.title('Time performance comparison: IDA* vs BFS')
    plt.legend()

    plt.grid(True, which="both", ls="--", linewidth=0.5)
    plt.savefig("plots/IDA_vs_BFS_no_log")
    plt.show()


# Different way of comparing IDA* vs BFS
def plot_compare_algs():
    scramble_lengths = []
    ida_times = []
    bfs_times = []
    
    # Plot the data
    plt.figure(figsize=(12, 8))
    
    for i in range(0, 20):
        # Read the CSV file
        data_bfs = pd.read_csv(f"csv_files/bfs_performance/bfs_performance_{i}.csv")
        data_ida = pd.read_csv(f"csv_files/ida_performance/ida_performance_new_heuristics{i}.csv")
        
        # Extract columns
        scramble_lengths.append(data_bfs["Scramble Length"].values)
        ida_times.append(data_ida["IDA* Time (s)"].values)
        ida_times[i] = ida_times[i][:len(data_bfs)]
        bfs_times.append(data_bfs["BFS Time (s)"].values)
        
        # Plot individual instances
        plt.plot(scramble_lengths[i], ida_times[i], color='blue', alpha=0.5, marker='o', label='IDA* Time' if i == 0 else "")
        plt.plot(scramble_lengths[i], bfs_times[i], color='orange', alpha=0.5, marker='o', label='BFS Time' if i == 0 else "")
    
    # Compute average values across all instances
    avg_scramble_lengths = np.unique(np.concatenate(scramble_lengths))
    avg_ida_times = []
    avg_bfs_times = []

    for length in avg_scramble_lengths:
        # Calculate averages for each scramble length
        ida_avg = np.mean([ida_time[np.where(scramble_length == length)[0][0]] 
                           for ida_time, scramble_length in zip(ida_times, scramble_lengths) 
                           if length in scramble_length])
        bfs_avg = np.mean([bfs_time[np.where(scramble_length == length)[0][0]] 
                           for bfs_time, scramble_length in zip(bfs_times, scramble_lengths) 
                           if length in scramble_length])
        avg_ida_times.append(ida_avg)
        avg_bfs_times.append(bfs_avg)

    # Plot averages
    plt.plot(avg_scramble_lengths, avg_ida_times, color='darkblue', marker='o', linewidth=3, label='Average IDA* Time')
    plt.plot(avg_scramble_lengths, avg_bfs_times, color='darkorange', marker='o', linewidth=3, label='Average BFS Time')
    plt.yscale('log') 
    # Add labels, title, and legend
    plt.title(f"Performance Comparison of 20 IDA* and BFS Instances", fontsize=16)
    plt.xlabel('Scramble Length', fontsize=14)
    plt.ylabel('Time (seconds)', fontsize=14)
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.legend(fontsize=12)
    
    # Save the plot
    plt.tight_layout()
    plt.savefig("plots/IDA_vs_BFS_performance_log.png")

# Performance of IDA*
def plot_ida():
    scramble_lengths = []
    ida_times = []
    
    # Plot the data
    plt.figure(figsize=(12, 8))
    
    for i in range(0, 20):
        # Read the CSV file
        data = pd.read_csv(f"csv_files/ida_performance/ida_performance_new_heuristics{i}_sclen8.csv")
        
        # Extract columns
        scramble_lengths.append(data["Scramble Length"].values)
        ida_times.append(data["IDA* Time (s)"].values)
        
        # Plot individual instances with light blue color
        plt.plot(scramble_lengths[i], ida_times[i], color='lightblue', alpha=0.5, marker='o', label='IDA* Instance' if i == 0 else "")


    # Compute average values across all instances
    avg_scramble_lengths = np.unique(np.concatenate(scramble_lengths))
    avg_ida_times = []

    for length in avg_scramble_lengths:
        # Calculate averages for each scramble length
        avg_time = np.mean([ida_time[np.where(scramble_length == length)[0][0]] 
                            for ida_time, scramble_length in zip(ida_times, scramble_lengths) 
                            if length in scramble_length])
        avg_ida_times.append(avg_time)

    # Plot average performance with dark blue color
    plt.plot(avg_scramble_lengths, avg_ida_times, color='darkblue', marker='o', linewidth=2, label='Average IDA* Time')
    
    # Add labels, title, and legend
    plt.title(f"Performance Comparison of 20 IDA* Instances (new heuristics)", fontsize=16)
    plt.xlabel('Scramble Length', fontsize=14)
    plt.ylabel('Time (seconds)', fontsize=14)
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.legend(fontsize=12)
    
    # Save the plot
    plt.tight_layout()
    plt.savefig("plots/IDA_performance_new_heuristics_8.png")

# Plot of IDA* performance depending on depth
def plot_time_vs_ida_depth():
    data = pd.read_csv(f"csv_files/real_experiments/performance_ida_depth_vs_time_8.csv")
    # Parse the data into a DataFrame
        
    # Ensure data contains the expected columns
    if not {"ida_depth", "Time (s)"}.issubset(data.columns):
        raise ValueError("The DataFrame must contain 'Scramble Length' and 'Solution Time (s)' columns.")

    # Convert data types
    data["ida_depth"] = data["ida_depth"].astype(int)
    data["Time (s)"] = data["Time (s)"].astype(float)

    plt.figure(figsize=(12, 8))

    # Compute and plot average values across all instances
    avg_data = data.groupby("ida_depth")["Time (s)"].mean().reset_index()
    plt.plot(
        avg_data["ida_depth"],
        avg_data["Time (s)"],
        color="blue",
    )
    # Customize the plot
    plt.title("Solution Time vs. IDA* max depth for twophase algorithm, scarmble len=30")
    plt.xlabel("IDA_Depth")
    plt.ylabel("Solution Time (s)")
    plt.xticks(np.arange(0,11))
    plt.grid(True)
    # Save the plot
    plt.tight_layout()
    plt.savefig("plots/time_performance_vs_ida_depth_8.png")
    plt.close()

# Performance of BFS
def plot_bfs():
    scramble_lengths = []
    bfs_times = []
    
    # Plot the data
    plt.figure(figsize=(16, 9))
    
    for i in range(0, 20):
        # Read the CSV file
        data = pd.read_csv(f"csv_files/bfs_performance/bfs_performance_{i}.csv")
        
        # Extract columns
        scramble_lengths.append(data["Scramble Length"].values)
        bfs_times.append(data["BFS Time (s)"].values)
        
        # Plot individual instances with light orange color
        plt.plot(scramble_lengths[i], bfs_times[i], color='orange', alpha=0.5, marker='o', label='BFS Instance' if i == 0 else "")
    
    # Compute average values across all instances
    avg_scramble_lengths = np.unique(np.concatenate(scramble_lengths))
    avg_bfs_times = []

    for length in avg_scramble_lengths:
        # Calculate averages for each scramble length
        avg_time = np.mean([bfs_time[np.where(scramble_length == length)[0][0]] 
                            for bfs_time, scramble_length in zip(bfs_times, scramble_lengths) 
                            if length in scramble_length])
        avg_bfs_times.append(avg_time)

    # Plot average performance with dark orange color
    plt.plot(avg_scramble_lengths, avg_bfs_times, color='darkorange', marker='o', linewidth=2, label='Average BFS Time')
    
    # Add labels, title, and legend
    plt.title(f"Performance Comparison of 20 BFS Instances", fontsize=16)
    plt.xlabel('Scramble Length', fontsize=14)
    plt.ylabel('Time (seconds)', fontsize=14)
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.legend(fontsize=12)
    
    # Save the plot
    plt.tight_layout()
    plt.savefig("plots/BFS_performance.png")
    plt.close()

def plot_solution_times():
    """
    Plots solution time (in milliseconds) vs. scramble length for each instance with enhanced readability.
    """
    # Load the data
    data_generic = pd.read_csv("csv_files/real_experiments/Performance_2phase_02_02_HQ.csv")
    data_ida = pd.read_csv("csv_files/real_experiments/Performance_2phaseida_02_02_depth_06.csv")
    
    # Ensure data contains the expected columns
    required_columns = {"Scramble Length", "Solution Time (s)"}
    if not required_columns.issubset(data_generic.columns):
        raise ValueError("data_generic must contain 'Scramble Length' and 'Solution Time (s)' columns.")
    if not required_columns.issubset(data_ida.columns):
        raise ValueError("data_ida must contain 'Scramble Length' and 'Solution Time (s)' columns.")

    # Convert data types
    data_generic["Scramble Length"] = data_generic["Scramble Length"].astype(int)
    data_generic["Solution Time (s)"] = data_generic["Solution Time (s)"].astype(float)
    data_ida["Scramble Length"] = data_ida["Scramble Length"].astype(int)
    data_ida["Solution Time (s)"] = data_ida["Solution Time (s)"].astype(float)

    # Compute average solution times per scramble length
    avg_data_generic = data_generic.groupby("Scramble Length")["Solution Time (s)"].mean().reset_index()
    avg_data_ida = data_ida.groupby("Scramble Length")["Solution Time (s)"].mean().reset_index()

    # Convert average times from seconds to milliseconds
    avg_data_generic["Solution Time (ms)"] = avg_data_generic["Solution Time (s)"] * 1000
    avg_data_ida["Solution Time (ms)"] = avg_data_ida["Solution Time (s)"] * 1000

    # Create the plot with a larger figure size
    plt.figure(figsize=(12, 8))
    
    # Plot the average solution times (in ms) with markers and thicker lines
    plt.plot(
        avg_data_generic["Scramble Length"],
        avg_data_generic["Solution Time (ms)"],
        color="blue",
        marker='o',
        linestyle='-',
        linewidth=2,
        markersize=6,
        label="Two-Phase"
    )
    plt.plot(
        avg_data_ida["Scramble Length"],
        avg_data_ida["Solution Time (ms)"],
        color="orange",
        marker='o',
        linestyle='-',
        linewidth=2,
        markersize=6,
        label="Two-Phase + IDA* depth 6"
    )
    
    # Customize the plot for better readability
    plt.title("Solution Time vs. Scramble Length", fontsize=20, fontweight='bold')
    plt.xlabel("Scramble Length", fontsize=16)
    plt.ylabel("Solution Time (ms)", fontsize=16)
    
    # Set tick labels to a larger font size (18pt)
    plt.xticks(np.arange(0, 21), fontsize=18)
    plt.yticks(fontsize=18)
    
    plt.legend(fontsize=18)
    plt.grid(True)
    
    # Adjust layout and save the plot with high resolution (300 dpi)
    plt.tight_layout()
    plt.savefig("plots/Performance_time_02_02_2phase_depth_06.png", dpi=300)
    plt.close()


def plot_solution_lengths():
    """
    Plots solution length vs. scramble length for each instance with enhanced readability.
    """
    # Load the data
    data_generic = pd.read_csv("csv_files/real_experiments/Performance_2phase_02_02_HQ.csv")
    data_ida = pd.read_csv("csv_files/real_experiments/Performance_2phaseida_02_02_depth_06.csv")

    # Ensure data contains the expected columns
    required_columns = {"Scramble Length", "Solution Moves"}
    if not required_columns.issubset(data_generic.columns):
        raise ValueError("data_generic must contain 'Scramble Length' and 'Solution Moves' columns.")
    if not required_columns.issubset(data_ida.columns):
        raise ValueError("data_ida must contain 'Scramble Length' and 'Solution Moves' columns.")
    
    # Convert data types
    data_generic["Scramble Length"] = data_generic["Scramble Length"].astype(int)
    data_generic["Solution Moves"] = data_generic["Solution Moves"].astype(float)
    data_ida["Scramble Length"] = data_ida["Scramble Length"].astype(int)
    data_ida["Solution Moves"] = data_ida["Solution Moves"].astype(float)
    
    # Compute average solution moves per scramble length
    avg_data_generic = data_generic.groupby("Scramble Length")["Solution Moves"].mean().reset_index()
    avg_data_ida = data_ida.groupby("Scramble Length")["Solution Moves"].mean().reset_index()

    # Create the plot with a larger figure size
    plt.figure(figsize=(12, 8))
    
    # Plot the average solution moves with markers and thicker lines for clarity
    plt.plot(
        avg_data_generic["Scramble Length"],
        avg_data_generic["Solution Moves"],
        color="blue",
        marker='o',
        linestyle='-',
        linewidth=2,
        markersize=6,
        label="Two-Phase"
    )
    plt.plot(
        avg_data_ida["Scramble Length"],
        avg_data_ida["Solution Moves"],
        color="orange",
        marker='o',
        linestyle='-',
        linewidth=2,
        markersize=6,
        label="Two-Phase + IDA* depth 6"
    )
    
    # Customize the plot for better readability
    plt.title("Solution Length vs. Scramble Length", fontsize=20, fontweight='bold')
    plt.xlabel("Scramble Length", fontsize=16)
    plt.ylabel("Solution Length", fontsize=16)
    
    # Set tick labels to a larger font size (18pt)
    plt.xticks(np.arange(0, 21), fontsize=18)
    # Assuming the y-axis range based on your data; adjust as needed
    plt.yticks(np.arange(min(avg_data_ida["Scramble Length"]), max(avg_data_ida["Scramble Length"]) + 1, 1), fontsize=18)
    
    plt.legend(fontsize=18)
    plt.grid(True)
    
    # Adjust layout and save the plot with high resolution (300 dpi)
    plt.tight_layout()
    plt.savefig("plots/Performance_len_02_02_2phase_depth_06.png", dpi=300)
    plt.close()

def plot_two_phase_len_and_time():
    # Load dataset
    data = pd.read_csv("csv_files/real_experiments/two_phase_times_and_solution_lengths.csv")

    # -------------------------
    # Plot 1: Solution Length Distribution
    # -------------------------
    avg_len = data["Length"].mean()
    plt.figure(figsize=(8, 5))
    plt.hist(data["Length"], bins=range(min(data["Length"]), max(data["Length"]) + 2),
             edgecolor='black', alpha=0.7, color='orange')
    plt.axvline(avg_len, color='red', linestyle='dashed', linewidth=2,
                label=f"Avg = {avg_len:.2f}")
    plt.xlabel("Solution Length", fontsize=16)
    plt.ylabel("Frequency", fontsize=16)
    plt.title("Solution Length Distribution", fontsize=22, fontweight='bold')
    plt.legend(fontsize=14)
    plt.grid(axis="y", linestyle="--", alpha=0.7)
    plt.gca().yaxis.set_major_formatter(mticker.FuncFormatter(lambda x, _: f"{x/1000:.0f}k"))
    plt.tight_layout()
    plt.savefig("plots/two_phase_solution_length.png", dpi=300)
    plt.show()
    plt.close()

    # -------------------------
    # Plot 2: Solution Time Distribution (in ms)
    # -------------------------
    # Convert time values from seconds to milliseconds
    time_ms = data["Time (ms)"] * 1000
    avg_time = time_ms.mean()
    plt.figure(figsize=(8, 5))
    sns.histplot(time_ms, bins=40, kde=True, color="orange", edgecolor="black",
                 log_scale=True)
    plt.axvline(avg_time, color='red', linestyle='dashed', linewidth=2,
                label=f"Avg = {avg_time:.2f} ms")
    plt.xlabel("Time (ms)", fontsize=16)
    plt.ylabel("Frequency", fontsize=16)
    plt.title("Solution Time Distribution", fontsize=22, fontweight='bold')
    plt.legend(fontsize=14)
    plt.grid(axis="y", linestyle="--", alpha=0.7)
    plt.gca().yaxis.set_major_formatter(mticker.FuncFormatter(lambda x, _: f"{x/1000:.0f}k"))
    plt.tight_layout()
    plt.savefig("plots/two_phase_solution_time.png", dpi=300)
    plt.show()
    plt.close()


if __name__ == "__main__":
    plot_solution_lengths()
    plot_solution_times()