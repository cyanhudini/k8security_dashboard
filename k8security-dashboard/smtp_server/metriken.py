def add(x, y):
    return x+y

def subtract(x, y):
    result = x-y
    return result

def multiply(a,b):

    return a*b

def divide(x, y):
    if y == 0:
        print("Error! Division by zero.")
        return None
    else:
        return x / y

# Unbenutzte Variable
unused_variable = 11
d = 12
# Schlechte Benennung und fehlende Dokumentation
def complex_operation(data_list):
    temp = 0
    for i in data_list:
        temp = temp + i
    return temp