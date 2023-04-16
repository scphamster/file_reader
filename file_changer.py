import argparse
import logging
import time
import string
import random

WORD_LEN = 10


def word_generator():
    return "sending " + str(random.randint(0, 1000)) + " sent " + str(random.randint(0, 1000)) + '\n'


def main():
    file = "./test.txt"

    while True:
        f = open(file, 'a');
        word = word_generator()
        print(word)
        f.write(word)
        f.close()
        time.sleep(1)


if __name__ == '__main__':
    logging.basicConfig(level=logging.INFO)
    main()
