import matplotlib.pyplot as plt

def experiment8a_plot():
    x = [x for x in range(1, 32_769)]
    for b in [4,5,6,7]:
        plt.clf()
        plt.xlabel('n')
        plt.ylabel('estimated n value / n')
        plt.title(f"HLL: N estimation test for {b} bits")
        for (hash_name, color) in [("sha1", 'or'), ("blake2", 'ob'), ("my_hash", 'og')]:           
            filename = f"data/exp8a_{hash_name}_b_{b}.csv"
            file = open(filename, 'r')
            lines = file.readlines()
            y = [ float((line.split(";"))[1]) for line in lines ]
            label = f"Hash = {hash_name}"
            plt.plot(x, y, color, label=label, markersize=1)
        plt.legend()
        plt.savefig(f'data/exp8a_b_{b}_plot.png')


def experiment8b_plot():
    x = [x for x in range(1, 32_769)]
    for (b, k) in [(5,5),(6,10),(7,20)]:
        plt.clf()
        plt.xlabel('n')
        plt.ylabel('estimated n value / n')
        plt.title(f"HLL vs MC: N estimation test for {2 ** b} registers and {k} hashes")
        
        filename = f"data/exp8b_blake2_k_{k}.csv"
        file = open(filename, 'r')
        lines = file.readlines()
        y = [ float((line.split(";"))[1]) for line in lines ]
        label = f"MC(k={k})"
        plt.plot(x, y, "or", label=label, markersize=1)
        
        filename = f"data/exp8a_blake2_b_{b}.csv"
        file = open(filename, 'r')
        lines = file.readlines()
        y = [ float((line.split(";"))[1]) for line in lines ]
        label = f"HLL(m={2** b})"
        plt.plot(x, y, "og", label=label, markersize=1)

        plt.legend()
        plt.savefig(f'data/exp8b_b_{b}_k_{k}_plot.png')
    
if __name__ == "__main__":
    experiment8b_plot()
