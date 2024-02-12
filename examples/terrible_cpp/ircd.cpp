#include <string>
#include <vector>

namespace {
    std::vector<std::string> channels;
    std::vector<std::string> users;
}

extern "C" void add_channel (const char * channel   ) { channels.push_back(channel); }
extern "C" void add_user    (const char * user      ) { users   .push_back(user   ); }

extern "C" void for_each_channel(void (*per_channel)(const char * channel)) {
    for (const auto & channel : channels) (*per_channel)(channel.c_str());
}

extern "C" void for_each_user(void (*per_user)(const char * user)) {
    for (const auto & user : users) (*per_user)(user.c_str());
}
