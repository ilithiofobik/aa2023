import matplotlib.pyplot as plt

def experiment5b_plot():
    for (k, color) in [(2, 'or'), (3, 'ob'), (10, 'og'), (100, 'oc'), (400, 'om')]:
        x = [x for x in range(1, 10_001)]
        plt.clf()
        plt.xlabel('n')
        plt.ylabel('estimated n value / n')
        plt.title("N estimation test")
        filename = f"data/exp5b_k_{k}.csv"
        file = open(filename, 'r')
        lines = file.readlines()
        y = [ float((line.split(";"))[1]) for line in lines ]
        label = f"k = {k}"
        plt.plot(x, y, color, label=label, markersize=1)
        plt.legend()
        plt.savefig(f'data/exp5_k_{k}_plot.png')

def experiment6_plot():
    x = [x for x in range(1, 10_001)]
    for b in [8,16,32,48]:
        plt.clf()
        plt.xlabel('n')
        plt.ylabel('estimated n value / n')
        plt.title(f"N estimation test for {b} bits")
        for (hash_name, color) in [("sha1", 'or'), ("blake2", 'ob'), ("my_hash", 'og')]:           
            filename = f"data/exp6_{hash_name}_b_{b}.csv"
            file = open(filename, 'r')
            lines = file.readlines()
            y = [ float((line.split(";"))[1]) for line in lines ]
            label = f"Hash = {hash_name}"
            plt.plot(x, y, color, label=label, markersize=1)
        plt.legend()
        plt.savefig(f'data/exp6_b_{b}_plot.png')


def experiment7_plot():
    x = [x for x in range(1, 10_001)]
    for alpha in [0.05, 0.01, 0.005]:
        plt.clf()
        plt.xlabel('n')
        plt.ylabel('estimated n value / n')
        plt.title(f"N estimation with bound for alpha = {alpha}")
        for (bound_type, color) in [("chernoff",'r'), ("chebyshev", 'g'), ("simulation", 'b')]:       
            filename = f"data/exp7_{bound_type}_alpha_{alpha}.txt"
            file = open(filename, 'r')
            y_val = float(file.read())
            label = f"Bound = {bound_type}"
            y = [1.0 + y_val for _ in range(1, 10_001)]
            plt.plot(x, y, color, label=label)
            y = [1.0 - y_val for _ in range(1, 10_001)]
            plt.plot(x, y, color)
        filename = f"data/exp5b_k_400.csv"
        file = open(filename, 'r')
        lines = file.readlines()
        y = [ float((line.split(";"))[1]) for line in lines ]
        plt.plot(x, y, "oc", markersize=1)
        plt.legend()
        plt.savefig(f'data/exp7_alpha_{alpha}_plot.png')
    
if __name__ == "__main__":
    #experiment5b_plot()
    #experiment6_plot()
    experiment7_plot()
