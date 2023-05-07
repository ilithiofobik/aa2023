import matplotlib.pyplot as plt

def experiment_sim_vs_anal():
    for n in [1, 3, 6, 12, 24, 48]:
        qs = [0.01 * x for x in range(5, 46)]
        plt.clf()
        plt.xlabel('q')
        plt.ylabel('P(n, q)')
        plt.title("P(n,q) for analytic methods and simulation")
        for (name, color) in [('grunspan', 'r'), ('nakamoto', 'b'), ('simulation', 'g')]: 
            filename = f"data/{name}_ppb_n_{n}.csv"
            file = open(filename, 'r')
            lines = file.readlines()
            y = [ float((line.split(";"))[1]) for line in lines ]
            plt.plot(qs, y, color, label=name, markersize=2)
        plt.legend()
        plt.savefig(f'data/ppn_n_{n}_plot.png')

def experiment_find_ppb():
    for ppb in [0.1, 0.01, 0.001]:
        qs = [0.01 * x for x in range(5, 46)]
        plt.clf()
        plt.xlabel('q')
        plt.ylabel('n')
        plt.title(f"minimal n for P(n,q)={ppb}")
        for (name, color) in [('grunspan', 'r'), ('nakamoto', 'b')]: 
            filename = f"data/{name}_find_n_ppb_{ppb}.csv"
            file = open(filename, 'r')
            lines = file.readlines()
            y = [ float((line.split(";"))[1]) for line in lines ]
            plt.plot(qs, y, color, label=name, markersize=2)
        plt.legend()
        plt.savefig(f'data/find_n_ppb_{ppb}_plot.png')
    
if __name__ == "__main__":
    experiment_sim_vs_anal()
    experiment_find_ppb()
    
