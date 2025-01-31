import pandas as pd
import matplotlib.pyplot as plt
import numpy as np


def plot_compare_algs():
    scramble_lengths = []
    ida_times = []
    bfs_times = []
    
    # Plot the data
    plt.figure(figsize=(12, 8))
    
    for i in range(0, 20):
        # Read the CSV file
        data_bfs = pd.read_csv(f"csv_files/bfs_performance/bfs_performance_{i}.csv")
        data_ida = pd.read_csv(f"csv_files/ida_performance/ida_performance_{i}.csv")
        
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
    
    # Add labels, title, and legend
    plt.title(f"Performance Comparison of 20 IDA* and BFS Instances", fontsize=16)
    plt.xlabel('Scramble Length', fontsize=14)
    plt.ylabel('Time (seconds)', fontsize=14)
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.legend(fontsize=12)
    
    # Save the plot
    plt.tight_layout()
    plt.savefig("plots/IDA_vs_BFS_performance.png")

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
    Plots solution time vs. scramble length for each instance.

    """
    data_generic = pd.read_csv(f"csv_files/real_experiments/performance_no_ida_it_10000.csv")
    data_ida = pd.read_csv(f"csv_files/real_experiments/performance_ida_it_10000.csv")
    # Parse the data into a DataFrame
        # Ensure data contains the expected columns
    if not {"Scramble Length", "Solution Time (s)"}.issubset(data_generic.columns):
        raise ValueError("The DataFrame must contain 'Scramble Length' and 'Solution Time (s)' columns.")
    if not {"Scramble Length", "Solution Time (s)"}.issubset(data_ida.columns):
        raise ValueError("The DataFrame must contain 'Scramble Length' and 'Solution Time (s)' columns.")

    # Convert data types
    data_generic["Scramble Length"] = data_generic["Scramble Length"].astype(int)
    data_generic["Solution Time (s)"] = data_generic["Solution Time (s)"].astype(float)
    data_ida["Scramble Length"] = data_ida["Scramble Length"].astype(int)
    data_ida["Solution Time (s)"] = data_ida["Solution Time (s)"].astype(float)

    plt.figure(figsize=(16, 9))

    # Compute and plot average values across all instances
    avg_data_generic = data_generic.groupby("Scramble Length")["Solution Time (s)"].mean().reset_index()
    plt.plot(
        avg_data_generic["Scramble Length"],
        avg_data_generic["Solution Time (s)"],
        color="blue",
        label="two phase standard"
    )
    avg_data_ida = data_ida.groupby("Scramble Length")["Solution Time (s)"].mean().reset_index()
    plt.plot(
        avg_data_ida["Scramble Length"],
        avg_data_ida["Solution Time (s)"],
        color="orange",
        label="two phase + ida"
    )
    # Customize the plot
    plt.title("Solution Time vs. Scramble Length for twophase algorithm")
    plt.xlabel("Scramble Length")
    plt.ylabel("Solution Time (s)")
    plt.xticks(np.arange(0,31))
    plt.legend()
    plt.grid(True)
    # Save the plot
    plt.tight_layout()
    plt.savefig("plots/time_performance_real_it_10000.png")
    plt.close()


def plot_solution_lengths():
    """
    Plots solution length vs. scramble length for each instance.
    """
    # Load the data
    data_generic = pd.read_csv(f"csv_files/real_experiments/performance_no_ida_it_10000.csv")
    data_ida = pd.read_csv(f"csv_files/real_experiments/performance_ida_it_10000.csv")



    # Ensure data contains the expected columns
    if not {"Scramble Length", "Solution Moves"}.issubset(data_generic.columns):
        raise ValueError("The DataFrame must contain 'Scramble Length' and 'Solution Moves' columns.")
    if not {"Scramble Length", "Solution Moves"}.issubset(data_generic.columns):
        raise ValueError("The DataFrame must contain 'Scramble Length' and 'Solution Moves' columns.")
    
    # Convert data types
    data_generic["Scramble Length"] = data_generic["Scramble Length"].astype(int)
    data_generic["Solution Moves"] = data_generic["Solution Moves"].astype(float)
    data_ida["Scramble Length"] = data_ida["Scramble Length"].astype(int)
    data_ida["Solution Moves"] = data_ida["Solution Moves"].astype(float)

    plt.figure(figsize=(16, 9))

    # Compute average solution moves per scramble length
    avg_data_generic = data_generic.groupby("Scramble Length")["Solution Moves"].mean().reset_index()
    # Plot the average solution moves
    plt.plot(
        avg_data_generic["Scramble Length"],
        avg_data_generic["Solution Moves"],
        color="blue",
        label="two phase standard"
    )
    avg_data_ida = data_ida.groupby("Scramble Length")["Solution Moves"].mean().reset_index()
    # Plot the average solution moves
    plt.plot(
        avg_data_ida["Scramble Length"],
        avg_data_ida["Solution Moves"],
        color="orange",
        label="two phase + ida"
    )
    # Customize the plot
    plt.title("Solution Length vs. Scramble Length for two phase algorithm")
    plt.xlabel("Scramble Length")
    plt.ylabel("Solution Length")
    plt.xticks(np.arange(0, 31))
    plt.yticks(np.arange(0, 22))
    plt.legend()
    plt.grid(True)

    # Save the plot
    plt.tight_layout()
    plt.savefig("plots/length_performance_real_it_10000.png")
    plt.close()




if __name__ == "__main__":
    plot_ida()