#include <iostream>
#include <openssl/md5.h>
using namespace std;

int main() {
    string input("yzbqklnj");
    int number = 0;
    unsigned char buffer[MD5_DIGEST_LENGTH];

    do {
        input.resize(8);
        input.append(to_string(++number));
        MD5((unsigned char*)input.c_str(), input.length(), buffer);
    }
    while (buffer[0] != 0 || buffer[1] != 0 || (buffer[2] & 0xF0) != 0);

    cout << number << endl;
}
