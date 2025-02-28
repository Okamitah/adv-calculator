import numpy as np 

def separate_matrix(expression):
    expression = expression.replace(' ','')
    if ';' in expression:
        expression = expression.split(';')
        for index,vector in enumerate(expression):
            expression[index] = vector.split(',')
    else:
        expression = expression.split(',')
    return np.array(expression)

operations_map = {
    '+': lambda x,y: x+y,
    '-': lambda x,y: x-y,
    '.': lambda x,y: x.dot(y),
    '*': lambda x,y: x*y,
    '**': lambda x,y: x+y,
    '..': lambda x,y: x+y,
    '/': lambda x,y: x+y,
}

def split_op(expression, operations):
    expression_list = []
    sep = 0
    for index, character in enumerate(expression):
        if character in operations:
            expression_list.append(expression[sep:index-1])
            expression_list.append(expression[index])
            sep = index+1 
    expression_list.append(expression[sep:])
    return expression_list

def separate_operation(expression):
    expression = expression.replace(' ','')
    expression = expression.split('+')
    return expression

print(separate_matrix("1, 5  ,2 ; 2,4,8")) 

print("\nWelcome to my matrix calculator!\n")
print("Allowed operators: +, -, * and /\n")
exp = ""
while exp != "exit":
    exp = input("> ")
    print(separate_matrix(exp))
