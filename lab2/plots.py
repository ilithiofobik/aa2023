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
    
if __name__ == "__main__":
    experiment5b_plot()
