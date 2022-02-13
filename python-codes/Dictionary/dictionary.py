import json
from difflib import get_close_matches

z = 'y'
while (z == 'y' or z == 'Y'):
    data = json.load(open("data.json"))
    def translate(check):
        if check in data:
            return data[check]
        elif check.title() in data:
            return data[check.title()]
        elif check.upper() in data:
            return data[check.upper()]
        elif len(get_close_matches(check, data.keys())) > 0 :
            print("Did you mean '%s' instead" %get_close_matches(check, data.keys())[0])
            decide = input("Press Y for yes and N for no \n")
            if (decide == 'Y' or decide == 'y'):
                return data[get_close_matches(check, data.keys())[0]]
            elif (decide == 'n' or decide == 'N'):
                print("OK, please`re-enter the corect word.")
            else:
                print("you entered wrong key please enter either y or n")
        else:
            print("\nYour entered a wrong word. please check again and re-enter :")

    word = input("\nEnter the word you want to know the meaning of: ")
    output = translate(word)
    if type(output) == list:
        for item in output:
            print(item)

    z = input("Do you want to repeat: y or n ")
    if (z == 'y' or z == 'Y'):
        continue
    else:
        break
