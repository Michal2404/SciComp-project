import pandas as pd
import matplotlib.pyplot as plt

def plot_compare():
    # Read the CSV file
    data = pd.read_csv("csv_files/algorithm_comparison.csv")

    # Extract columns
    scramble_lengths = data["Scramble Length"]
    bfs_times = data["BFS Time (s)"]
    ida_times = data["IDA* Time (s)"]

    # Plot the data
    plt.figure(figsize=(10, 6))
    plt.plot(scramble_lengths, bfs_times, marker='o', label='BFS Time')
    plt.plot(scramble_lengths, ida_times, marker='o', label='IDA* Time')

    # Add labels, title, and legend
    plt.title('BFS vs IDA* Time Comparison', fontsize=16)
    plt.xlabel('Scramble Length', fontsize=14)
    plt.ylabel('Time (seconds)', fontsize=14)
    plt.xticks(scramble_lengths)
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.legend(fontsize=12)

    # Show the plot
    plt.tight_layout()
    plt.show()

def plot_ida():
    scramble_lengths = []
    ida_times = []
    # Plot the data
    plt.figure(figsize=(10, 6))
    for i in range(0, 20):
        # Read the CSV file
        data = pd.read_csv(f"csv_files/ida_performance_{i}.csv")

        # Extract columns
        scramble_lengths.append(data["Scramble Length"])
        ida_times.append(data["IDA* Time (s)"])

        
        plt.plot(scramble_lengths[i], ida_times[i], marker='o', label='IDA* Time')

    # Add labels, title, and legend
    plt.title('IDA* Performance', fontsize=16)
    plt.xlabel('Scramble Length', fontsize=14)
    plt.ylabel('Time (seconds)', fontsize=14)
    #plt.xticks(scramble_lengths)
    plt.grid(True, linestyle='--', alpha=0.7)
    plt.legend(fontsize=12)

    # Show the plot
    plt.tight_layout()
    plt.show()

if __name__ == "__main__":
    plot_compare()
    plot_ida()