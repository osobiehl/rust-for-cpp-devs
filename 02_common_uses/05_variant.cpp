#include <iostream>
#include <string>
#include <variant>
#include <cassert>

class IpAddrKind {
public:
    struct V4 {
        uint8_t a, b, c, d;
    };

    struct V6 {
        std::string address;
    };

    using Variant = std::variant<V4, V6>;

    explicit IpAddrKind(V4 v4) : value(v4) {}
    explicit IpAddrKind(V6 v6) : value(v6) {}

    bool is_loopback() const {
        return std::visit([](const auto& addr) -> bool {
            if constexpr (std::is_same_v<std::decay_t<decltype(addr)>, V4>) {
                return addr.a == 127 && addr.b == 0 && addr.c == 0 && addr.d == 1;
            } else if constexpr (std::is_same_v<std::decay_t<decltype(addr)>, V6>) {
                return addr.address == "::1";
            } else {
                return false;
            }
        }, value);
    }

private:
    Variant value;
};

int main() {
    using V4 = IpAddrKind::V4;
    using V6 = IpAddrKind::V6;

    IpAddrKind localhostV4(V4{127, 0, 0, 1});
    IpAddrKind localhostV6(V6{"::1"});

    assert(localhostV4.is_loopback() && "incorrect localhost for IPv4");
    assert(localhostV6.is_loopback() && "incorrect localhost for IPv6");

    return 0;
}