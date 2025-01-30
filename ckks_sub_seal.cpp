// Compile command: g++ -std=c++17 -I./SEAL/install/include/SEAL-4.1 -o ckks_sub_seal ckks_sub_seal.cpp ./SEAL/install/lib/libseal-4.1.a

#include "seal/seal.h"

using namespace std;
using namespace seal;

#include <iostream>
#include <sstream>
#include <cstdint>

inline std::string uint64_to_hex_string(std::uint64_t value);

int main(int argc, char *argv[])
{

    double x = 1.1;
    double y = 2.2;

    if (argc > 1)
    {
        std::istringstream iss(argv[1]);
        iss >> x;
    }
    if (argc > 2)
    {
        std::istringstream iss(argv[2]);
        iss >> y;
    }

    EncryptionParameters parms(scheme_type::ckks);

    size_t poly_modulus_degree = 8192;
    parms.set_poly_modulus_degree(poly_modulus_degree);
    parms.set_coeff_modulus(CoeffModulus::Create(poly_modulus_degree, {60, 40, 40, 60}));

    SEALContext context(parms);

    KeyGenerator keygen(context);
    auto secret_key = keygen.secret_key();
    PublicKey public_key;
    keygen.create_public_key(public_key);

    Encryptor encryptor(context, public_key);
    Evaluator evaluator(context);
    Decryptor decryptor(context, secret_key);

    CKKSEncoder encoder(context);
    size_t slot_count = encoder.slot_count();
    cout << "CKKS Adding Two Numbers with Homomorphic Encryption" << endl;
    cout << "Number of slots: " << slot_count << endl
         << endl;

    vector<double> input;
    input.reserve(1);
    input.push_back(x);

    vector<double> input2;
    input2.reserve(1);
    input2.push_back(y);

    double scale = pow(2.0, 30);

    Plaintext a, b, c;

    Plaintext xPlain;
    encoder.encode(input, scale, xPlain);
    Ciphertext xEncrypted;
    encryptor.encrypt(xPlain, xEncrypted);

    Plaintext yPlain;
    encoder.encode(input2, scale, yPlain);
    Ciphertext yEncrypted;
    encryptor.encrypt(yPlain, yEncrypted);

    Ciphertext res;
    evaluator.sub(xEncrypted, yEncrypted, res);

    Plaintext plainResult;
    decryptor.decrypt(res, plainResult);

    vector<double> result;
    result.reserve(1);
    encoder.decode(plainResult, result);

    std::cout.precision(6);

    cout << x << "+" << y << "=" << result[0];
}
