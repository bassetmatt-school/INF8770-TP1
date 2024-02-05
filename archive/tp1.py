import time
import numpy as np
FILES = [f"texte_{i}.txt" for i in range(1, 7)]
PATH = "../data/textes/"


def load_file(index: int) -> str:
    if index < 0 or index >= len(FILES):
        raise ValueError("Index out of range")
    with open(PATH+FILES[index], "r") as f:
        msg = f.read()
    return msg


def init_dict(msg: str) -> dict[str, str]:
    dict_symb = {}
    n_symb = 0
    for i in range(len(msg)):
        if msg[i] not in dict_symb:
            dict_symb[msg[i]] = f"{n_symb:b}"
            n_symb += 1
    return dict_symb


def dict_update_size(dict_symb: dict[str, str]):
    n_symb = len(dict_symb)
    for k, v in dict_symb.items():
        dict_symb[k] = v.zfill(int(np.ceil(np.log2(n_symb))))


def log_size(x): return np.ceil(np.log2(x))


def compress(msg: str, dict_symb: dict[str, str]) -> (list[str], int):
    i, length = 0, 0
    n_symb = len(dict_symb)
    coded_msg = []
    while i < len(msg):
        # Next coded string
        next_str = msg[i]
        # Same, but with extra character (for the dictionary)
        next_str_extra = msg[i]

        # Tries to fit the largest string possible in the dictionary
        while next_str_extra in dict_symb and i < len(msg):
            i += 1
            next_str = next_str_extra
            if i < len(msg):  # If there is still characters to read
                next_str_extra += msg[i]

        # Coding of the string
        bin_code = dict_symb[next_str]
        coded_msg.append(bin_code)
        length += len(bin_code)
        # Adding the new string to the dictionary
        if i < len(msg):
            dict_symb[next_str_extra] = f"{n_symb:b}"
            n_symb += 1

        # Updating symbols size if necessary
        if log_size(n_symb) > len(coded_msg[-1]):
            dict_update_size(dict_symb)
    return coded_msg, length


def run(index: int, verbose: int = 1) -> dict[str, str]:
    start = time.time()
    msg = load_file(index)
    loaded = time.time()
    if verbose > 1:
        print(f"Message: {msg}")
    dict_symb = init_dict(msg)
    initial_length = int(log_size(len(dict_symb))*len(msg))
    dict_update_size(dict_symb)
    dict_loaded = time.time()
    if verbose > 1:
        print(f"Binary symbols: {dict_symb}")
        print(f"Initial length: {initial_length}")
    coded_msg, length = compress(msg, dict_symb)
    compressed = time.time()
    with open("../out/python_comp.bin", "w") as f:
        f.write("".join(coded_msg))
    if verbose > 2:
        print(f"Coded message: {coded_msg}")
    if verbose > 1:
        print(f"Or : {''.join(coded_msg)}")

    if verbose > 2:
        print(f"Final dictionary: {dict_symb}")
        print("")
    if verbose > 0:
        print(f"Length = {length}")
        print(f"Original length = {initial_length}")
        print(f"Compression rate = {100 - length/initial_length*100:.2f}%")
        print(f"Compression factor = {initial_length/length:.2f}")

    if verbose > 0:
        print("")
        print(f"Loading time: {(loaded-start)*10**3:.2f}ms")
        print(f"Dictionary loading time: {(dict_loaded-loaded)*10**3:.2f}ms")
        print(f"Compression time: {(compressed-dict_loaded)*10**3:.2f}ms")
        print(f"Total time: {(compressed-start)*10**3:.2f}ms")
    return dict_symb


def optimal_repeat_counter(repeats: list[int], dict_size: int) -> int:
    # Symbols can be repeated at most max_k times
    max_k = int(log_size(max(repeats)))
    symbol_size = log_size(dict_size)
    waste_list = []
    # Test all the k values for bit size encoding
    for k in range(1, max_k+1):
        # Bits wasted by non optimal encoding
        wasted = 0
        for x in repeats:
            # Number of bits to optimally count x
            opti_bits = log_size(x)
            # Optimal case
            if (opti_bits == k):
                continue
            # k too big, wasted bits = difference
            elif (opti_bits < k):
                wasted += k - opti_bits
            # k too low
            else:
                # We are going to use (x/2**k + 1) repetitions, and each repetition
                # is encoded on k bits + symbol_size bits for the symbol
                used = (np.floor(x / 2**k) + 1) * (k + symbol_size)
                # Ideally, we would have used opti_bits + symbol_size bits
                ideal = opti_bits + symbol_size
                # Wasted bits are the difference
                wasted += used - ideal
        waste_list.append(wasted)
    # Retrieve the k value that minimizes the wasted bits
    waste_list = np.asarray(waste_list, dtype=np.int32)
    return np.argmin(waste_list)+1


if __name__ == "__main__":
    # dict_symb = run(4, verbose=1)

    L = [4, 5, 6, 7, 3, 2, 1, 7, 7, 3, 4, 2, 3, 4, 5, 4, 3, 2]
    n_symb = 5
    k = optimal_repeat_counter(L, n_symb)
    print(k)
