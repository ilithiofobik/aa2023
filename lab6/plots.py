import matplotlib.pyplot as plt

def ex14_plots():
    ts = [x for x in range(0, 25)]
    plt.clf()
    plt.xlabel('t')
    plt.ylabel(f'|pi_t-pi_t * P|')
    plt.title(f"|pi_t-pi_t * P|for different alphas")
    for (alpha, color) in [(0.0, "r"), (0.25, "g"), (0.5, "b"), (0.75, "c"), (0.85, "y"), (1.0, "m")]: 
        filename = f"data/convergence_alpha_{alpha}.txt"
        file = open(filename, 'r')
        lines = file.readlines()
        y = [ float((line.split(";"))[1]) for line in lines ]
        plt.plot(ts, y, color, label=f"Alpha = {alpha}", markersize=2)
    plt.legend()
    plt.savefig(f'data/ex14_plots.png')


if __name__ == "__main__":
    ex14_plots()
