import numpy as np 

def separate_matrix(expression):
    if type(expression)==str:
        if ';' in expression:
            expr = [row.split(',') for row in expression.split(';')]
        else:
            expr = [expression.split(',')]  
    else:
        expr = expression
    arr = np.array(expr, dtype=float)
    return arr

operations_map = {
    '+': lambda x,y: x+y,
    '-': lambda x,y: x-y,
    '@': lambda x,y: x@y,
    '*': lambda x,y: x*y,
    '**': lambda x,y: x+y,
    '..': lambda x,y: x+y,
    '/': lambda x,y: x/y,
}

def split_op(expression:str, operations) -> list:

    expression = expression.replace(' ', '')
    expression_list = []
    sep = 0
    for index, character in enumerate(expression):
        if character in operations:
            expression_list.append(expression[sep:index])
            expression_list.append(expression[index])
            sep = index+1 
    expression_list.append(expression[sep:])
    return expression_list

def separate_expression(expression:str) -> list:
    operations = ('*','/','+','-','@')
    expr = split_op(expression,operations)
    return expr

def calculate(expression) -> float:
    expression = separate_expression(expression)
    addsub = ('+','-')
    muldiv = ('*','@','/')
    calc = []
    for index in range(1,len(expression),2):
        if expression[index] in muldiv:
            mat1 = separate_matrix(expression[index-1])
            mat2 = separate_matrix(expression[index+1])
            calc.append(operations_map[expression[index]](mat1,mat2))
        else:
            calc.append(expression[index-1])
            calc.append(expression[index])
    if expression[-2] in addsub:
        calc.append(expression[-1])
    for index in range(1,len(calc),2):
        mat1 = separate_matrix(calc[index-1])
        mat2 = separate_matrix(calc[index+1])
        calc[index+1] = operations_map[calc[index]](mat1,mat2)

    return calc[-1]


print(f"Separate matrix:{separate_matrix('1, 5  ,2 ; 2,4,8')}") 
print(f"Separate expression:{separate_expression('1, 5  ,2 ; 2,4*5,8 + 2,2,2;4,1*4,4')}") 
print(f"calculate expression:{calculate('1, 4,2+5,8,3 @ 3;3;3')}") 

print("\nWelcome to my matrix calculator!\n")
print("Allowed operators: +, -, * and /\n")
exp = ""
while exp != "e":
    exp = input("> ")
    print(calculate(exp))
