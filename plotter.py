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
        data = pd.read_csv(f"csv_files/ida_performance/ida_performance_{i}.csv")
        
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
    plt.title(f"Performance Comparison of 20 IDA* Instances", fontsize=16)
    plt.xlabel('Scramble Length', fontsize=14)
    plt.ylabel('Time (seconds)', fontsize=14)
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.legend(fontsize=12)
    
    # Save the plot
    plt.tight_layout()
    plt.savefig("plots/IDA_performance.png")

def plot_bfs():
    scramble_lengths = []
    bfs_times = []
    
    # Plot the data
    plt.figure(figsize=(12, 8))
    
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

if __name__ == "__main__":
    plot_compare_algs()
    plot_ida()
    plot_bfs()