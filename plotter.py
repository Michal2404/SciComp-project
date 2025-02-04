import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
import matplotlib.ticker as mticker

# Compare IDA* vs BFS
def plot_compare_ida_vs_bfs():
    data_dijkstra = pd.read_excel(f"src/data/dijkstra_analysis.xlsx")
    data_astar = pd.read_excel(f"src/data/astar_analysis.xlsx")

    dijkstra_means = data_dijkstra.groupby("scramble number of moves")["cross time (s)"].mean().reset_index()
    astar_means = data_astar.groupby("scramble number of moves")["cross time (s)"].mean().reset_index()

    plt.figure(figsize=(10,6))

    plt.plot(dijkstra_means["scramble number of moves"], dijkstra_means["cross time (s)"], '-o', label='Dijkstra')
    plt.plot(astar_means["scramble number of moves"], astar_means["cross time (s)"], '-s', label='A*')

    plt.yscale('log')

    plt.xlabel('Scramble Length')
    plt.ylabel('Times (s) [Log Scale]')
    plt.title('Time performance comparison: Dijkstra vs A*')
    plt.legend()

    plt.grid(True, which="both", ls="--", linewidth=0.5)
    plt.savefig("src/plots/Dijkstra_vs_A_no_log")
    plt.show()

# Plots time performance of CFOP
def plot_cfop_time():
    data_cfop = pd.read_excel(f"src/data/cfop_analysis.xlsx")

    cfop_means = data_cfop.groupby("scramble number of moves")["total time (ms)"].mean().reset_index()

    plt.figure(figsize=(10,6))

    plt.plot(cfop_means["scramble number of moves"], cfop_means["total time (ms)"], '-o', label='CFOP')

    plt.xlabel('Scramble Length')
    plt.ylabel('Times (ms)')
    plt.title('Time performance of CFOP')
    # plt.legend()

    plt.xticks(cfop_means["scramble number of moves"])

    plt.grid(True, which="both", ls="--", linewidth=0.5)
    plt.savefig("src/plots/CFOP_time")
    plt.show()

# Plots move performance of CFOP
def plot_cfop_moves():
    data_cfop = pd.read_excel(f"src/data/cfop_analysis.xlsx")

    cfop_means = data_cfop.groupby("scramble number of moves")["total number of moves"].mean().reset_index()

    plt.figure(figsize=(10,6))

    plt.plot(cfop_means["scramble number of moves"], cfop_means["total number of moves"], '-s', label='CFOP', color='orange')

    plt.xlabel('Scramble Length')
    plt.ylabel('Solution Length')
    plt.title('Time performance of CFOP')
    # plt.legend()

    plt.xticks(cfop_means["scramble number of moves"])

    plt.grid(True, which="both", ls="--", linewidth=0.5)
    plt.savefig("src/plots/CFOP_moves")
    plt.show()

def plot_cfop_time_and_moves():
    data_cfop = pd.read_excel("src/data/cfop_analysis.xlsx")

    cfop_means_time = data_cfop.groupby("scramble number of moves")["total time (ms)"].mean().reset_index()
    cfop_means_moves = data_cfop.groupby("scramble number of moves")["total number of moves"].mean().reset_index()

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 6))
    fig.suptitle('CFOP Time and Move performance', fontsize=16)

    # Plot CFOP time performance
    ax1.plot(cfop_means_time["scramble number of moves"], cfop_means_time["total time (ms)"], '-o', label='CFOP Time')
    ax1.set_xlabel('Scramble Length')
    ax1.set_ylabel('Times (ms)')
    ax1.set_title('Time performance of CFOP')
    # ax1.set_xticks(cfop_means_time["scramble number of moves"])
    ax1.grid(True, which="both", ls="--", linewidth=0.5)
    # Set x-axis to show integer ticks and divide ticks
    ax1.xaxis.set_major_locator(mticker.MultipleLocator(4))

    # Plot CFOP move performance
    ax2.plot(cfop_means_moves["scramble number of moves"], cfop_means_moves["total number of moves"], '-o', label='CFOP Moves', color='orange')
    ax2.set_xlabel('Scramble Length')
    ax2.set_ylabel('Total Moves')
    ax2.set_title('Move performance of CFOP')
    # ax2.set_xticks(cfop_means_moves["scramble number of moves"])
    ax2.grid(True, which="both", ls="--", linewidth=0.5)
    ax2.xaxis.set_major_locator(mticker.MultipleLocator(4))


    plt.tight_layout()
    plt.savefig("src/plots/CFOP_time_and_moves.png")
    plt.show()

def plot_stacked_bar_chart():
    data_cfop = pd.read_excel("src/data/cfop_analysis.xlsx")

    # Group data by scramble length and calculate mean times for each phase
    phase_time_means = data_cfop.groupby("scramble number of moves")[["cross time (ms)", "f2l time (ms)", "oll time (ms)", "pll time (ms)", "total time (ms)"]].mean().reset_index()
    phase_move_means = data_cfop.groupby("scramble number of moves")[["cross number of moves", "f2l number of moves", "oll number of moves", "pll number of moves", "total number of moves"]].mean().reset_index()

    # Calculate the total number of moves by summing the individual phase moves
    phase_move_means["total number of moves"] = (
        phase_move_means["cross number of moves"] +
        phase_move_means["f2l number of moves"] +
        phase_move_means["oll number of moves"] +
        phase_move_means["pll number of moves"]
    )

    # Normalize each phase's time and moves by the total time and total moves
    for phase in ["cross time (ms)", "f2l time (ms)", "oll time (ms)", "pll time (ms)"]:
        phase_time_means[phase] = (phase_time_means[phase] / phase_time_means["total time (ms)"]) * 100

    for phase in ["cross number of moves", "f2l number of moves", "oll number of moves", "pll number of moves"]:
        phase_move_means[phase] = (phase_move_means[phase] / phase_move_means["total number of moves"]) * 100

    # Plot stacked bar chart
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 6))
    fig.suptitle('Proportion of Time Spent and Moves Taken in Each Phase by Scramble Length', fontsize=16)

    # Define the phases and their colors
    phases_time = ["cross time (ms)", "f2l time (ms)", "oll time (ms)", "pll time (ms)"]
    phases_move = ["cross number of moves", "f2l number of moves", "oll number of moves", "pll number of moves"]
    colors = ["#1f77b4", "#ff7f0e", "#2ca02c", "#d62728"]

    # Plot each phase as a stacked bar for time
    bottom_time = None
    for phase, color in zip(phases_time, colors):
        if bottom_time is None:
            ax1.bar(phase_time_means["scramble number of moves"], phase_time_means[phase], label=phase.split(" ")[0], color=color)
            bottom_time = phase_time_means[phase]
        else:
            ax1.bar(phase_time_means["scramble number of moves"], phase_time_means[phase], label=phase.split(" ")[0], bottom=bottom_time, color=color)
            bottom_time += phase_time_means[phase]
    ax1.set_xlabel('Scramble Length')
    ax1.set_ylabel('Percentage of Total Time (%)')
    ax1.set_title('Time')
    ax1.xaxis.set_major_locator(mticker.MultipleLocator(4))
    ax1.legend(title="Phases", loc="lower right")
    # ax1.grid(True, which="both", ls="--", linewidth=0.5)

    # Plot each phase as a stacked bar for moves
    bottom_move = None
    for phase, color in zip(phases_move, colors):
        if bottom_move is None:
            ax2.bar(phase_move_means["scramble number of moves"], phase_move_means[phase], label=phase.split(" ")[0], color=color)
            bottom_move = phase_move_means[phase]
        else:
            ax2.bar(phase_move_means["scramble number of moves"], phase_move_means[phase], label=phase.split(" ")[0], bottom=bottom_move, color=color)
            bottom_move += phase_move_means[phase]
    ax2.set_xlabel('Scramble Length')
    ax2.set_ylabel('Percentage of Total Moves (%)')
    ax2.set_title('Moves')
    ax2.xaxis.set_major_locator(mticker.MultipleLocator(4))
    ax2.legend(title="Phases", loc="lower right")
    # ax2.grid(True, which="both", ls="--", linewidth=0.5)


    # # Plot stacked bar chart
    # fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 6))
    # fig.suptitle('Proportion of Time Spent and Moves Taken in Each Phase by Scramble Length', fontsize=16)
    # # fig, ax = plt.subplots(figsize=(10, 6))

    # # Define the phases and their colors
    # phases_time = ["cross time (ms)", "f2l time (ms)", "oll time (ms)", "pll time (ms)"]
    # phases_move = ["cross number of moves", "f2l number of moves", "oll number of moves", "pll number of moves"]
    # # print(phase_move_means)
    # # assert phase_time_means["total time (ms)"][0] == phase_time_means["cross time (ms)"][0] + phase_time_means["f2l time (ms)"][0] + phase_time_means["oll time (ms)"][0] + phase_time_means["pll time (ms)"][0]
    # colors = ["#1f77b4", "#ff7f0e", "#2ca02c", "#d62728"]

    # total_time = phase_time_means["cross time (ms)"] + phase_time_means["f2l time (ms)"] + phase_time_means["oll time (ms)"] + phase_time_means["pll time (ms)"]
    # total_move = phase_move_means["cross number of moves"] + phase_move_means["f2l number of moves"] + phase_move_means["oll number of moves"] + phase_move_means["pll number of moves"]

    # print(phase_time_means["total time (ms)"])
    # print(total_time)
    # # print(phase_move_means["total number of moves"])
    # # print(total_move)
    # # Plot each phase as a stacked bar time
    # bottom_time = None
    # for phase, color in zip(phases_time, colors):
    #     if bottom_time is None:
    #         ax1.bar(phase_time_means["scramble number of moves"], phase_time_means[phase]/total_time*100, label=phase.split(" ")[0], color=color)
    #         bottom_time = phase_time_means[phase]/total_time*100
    #     else:
    #         ax1.bar(phase_time_means["scramble number of moves"], phase_time_means[phase]/total_move*100, label=phase.split(" ")[0], bottom=bottom_time, color=color)
    #         bottom_time += phase_time_means[phase]/total_move*100
    # ax1.set_xlabel('Scramble Length')
    # ax1.set_ylabel('Percentage (%)')
    # ax1.set_title('Time')
    # # ax1.set_xticks(cfop_means_time["scramble number of moves"])
    # # ax1.grid(True, which="both", ls="--", linewidth=0.5)
    # # Set x-axis to show integer ticks and divide ticks
    # ax1.xaxis.set_major_locator(mticker.MultipleLocator(4))
    # ax1.legend(title="Phases", loc="lower right")

    # # Plot each phase as a stacked bar moves
    # bottom_move = None
    # for phase, color in zip(phases_move, colors):
    #     if bottom_move is None:
    #         ax2.bar(phase_move_means["scramble number of moves"], phase_move_means[phase]/phase_move_means["total number of moves"]*100, label=phase.split(" ")[0], color=color)
    #         bottom_move = phase_move_means[phase]/phase_move_means["total number of moves"]*100
    #     else:
    #         ax2.bar(phase_move_means["scramble number of moves"], phase_move_means[phase]/phase_move_means["total number of moves"]*100, label=phase.split(" ")[0], bottom=bottom_move, color=color)
    #         bottom_move += phase_move_means[phase]/phase_move_means["total number of moves"]*100
    # ax2.set_xlabel('Scramble Length')
    # ax2.set_ylabel('Percentage (%)')
    # ax2.set_title('Moves')
    # # ax2.set_xticks(cfop_means_moves["scramble number of moves"])
    # # ax2.grid(True, which="both", ls="--", linewidth=0.5)
    # ax2.xaxis.set_major_locator(mticker.MultipleLocator(4))
    # ax2.legend(title="Phases", loc="lower right")


    # # ax.set_xlabel('Scramble Length')
    # # ax.set_ylabel('Percentage (%)')
    # # ax.set_title('Proportion of Total Time Spent in Each Phase by Scramble Length')
    # # ax.xaxis.set_major_locator(mticker.MultipleLocator(4))
    # # ax.grid(True, which="both", ls="--", linewidth=0.5)

    plt.tight_layout()
    plt.savefig("src/plots/CFOP_phase_times_stacked_bar.png")
    plt.show()

def phase_times():
    data_cfop = pd.read_excel("src/data/cfop_analysis.xlsx")

    # Group data by scramble length and calculate mean times for each phase
    phase_means = data_cfop.groupby("scramble number of moves")[["cross time (ms)", "f2l time (ms)", "oll time (ms)", "pll time (ms)", "total time (ms)"]].mean().reset_index()

    # Define the phases and their colors
    phases = ["cross time (ms)", "f2l time (ms)", "oll time (ms)", "pll time (ms)"]
    colors = ["#1f77b4", "#ff7f0e", "#2ca02c", "#d62728"]

    # Plot each phase as a stacked bar
    bottom = None
    for phase, color in zip(phases, colors):
        percentage = phase_means[phase]/phase_means["total time (ms)"]*100
        print(f"{phase}: {percentage}%")
#--------------------------------------------------------------------------------------------------------------#

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
    # plot_compare_ida_vs_bfs()
    # plot_cfop_time()
    # plot_cfop_moves()
    # plot_cfop_time_and_moves()
    plot_stacked_bar_chart()
    # phase_times()